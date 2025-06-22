from pybricks.hubs import MoveHub
from pybricks.pupdevices import Motor, ColorDistanceSensor
from pybricks.parameters import Button, Color, Direction, Port, Side, Stop
from pybricks.robotics import DriveBase
from pybricks.tools import wait, StopWatch

# 定数
WHEEL_DIAMETER = 40  # 車輪の直径（mm）
WHEEL_CIRCUMFERENCE = 12566  # 円周（mm * 100、40 * 3.1416 ≈ 125.66）
DISTANCE_PER_DEGREE = 349  # WHEEL_CIRCUMFERENCE / 360 ≈ 0.3490667（mm * 1000 / deg）
AXLE_TRACK = 105  # 車軸間の距離（mm）
DISTANCE_THRESHOLD = 65  # 障害物検知判定閾値
MAX_DISTANCE_MEASUREMENT = 70  # 測定距離センサ最大値
DEFAULT_SPEED = 720  # デフォルトの速度（度/秒）
CALIBRATION_SAMPLES = 50  # キャリブレーションのサンプル数
COLLISION_THRESHOLD_Y = 2000  # Y軸閾値（左右）
COLLISION_THRESHOLD_Z = 2000  # Z軸閾値（前後、移動開始対策）
SPEED_THRESHOLD = 500  # 速度変化の閾値

hub = MoveHub()

# モーターとセンサー設定
motor_a = Motor(Port.A, positive_direction=Direction.COUNTERCLOCKWISE)
motor_b = Motor(Port.B, positive_direction=Direction.CLOCKWISE)
sensor = ColorDistanceSensor(Port.C)

# ロボットのドライブベースを設定
drive_base = DriveBase(motor_a, motor_b, wheel_diameter=WHEEL_DIAMETER, axle_track=AXLE_TRACK)

# キャリブレーション関数（静止時）
def calibrate_imu():
    print("キャリブレーション開始（静止時）...")
    ax_sum = ay_sum = az_sum = 0
    for _ in range(CALIBRATION_SAMPLES):
        ax, ay, az = hub.imu.acceleration()
        ax_sum += ax
        ay_sum += ay
        az_sum += az
        wait(10)
    ax_nominal = ax_sum // CALIBRATION_SAMPLES
    ay_nominal = ay_sum // CALIBRATION_SAMPLES
    az_nominal = az_sum // CALIBRATION_SAMPLES
    print("キャリブレーション完了: X =", ax_nominal, "Y =", ay_nominal, "Z =", az_nominal)
    return ax_nominal, ay_nominal, az_nominal

# 移動時のZ軸キャリブレーション
def calibrate_imu_moving():
    print("キャリブレーション開始（移動時）...")
    run_at(DEFAULT_SPEED)
    wait(500)  # 移動開始のスパイクを待つ
    az_sum = 0
    for _ in range(CALIBRATION_SAMPLES):
        _, _, az = hub.imu.acceleration()
        az_sum += az
        wait(10)
    run_at(0)  # 停止
    az_nominal_moving = az_sum // CALIBRATION_SAMPLES
    print("移動時キャリブレーション完了: Z =", az_nominal_moving)
    return az_nominal_moving

# 前進、後退
def run_at(speed=DEFAULT_SPEED):
    motor_a.run(speed)
    motor_b.run(speed)

def get_distance():
    return sensor.distance()

def turn(angle):
    global heading
    heading += angle
    drive_base.turn(angle)

# IMUデータの取得（移動平均）
def get_imu_accel_avg(samples=15):
    ax_sum = ay_sum = az_sum = 0
    for _ in range(samples):
        ax, ay, az = hub.imu.acceleration()
        ax_sum += ax
        ay_sum += ay
        az_sum += az
        wait(10)
    return ax_sum // samples, ay_sum // samples, az_sum // samples

# オドメトリによる位置と速度推定
def update_odometry():
    global px, py, pz, heading, last_left_angle, last_right_angle, vz
    left_angle = motor_a.angle()
    right_angle = motor_b.angle()
    left_delta = left_angle - last_left_angle
    right_delta = right_angle - last_right_angle
    last_left_angle = left_angle
    last_right_angle = right_angle
    distance = ((left_delta + right_delta) // 2) * DISTANCE_PER_DEGREE // 1000  # mm
    vz = -distance * 10  # 100ms周期で速度（mm/s）
    vz = max(min(vz, 1000), -1000)
    pz -= distance  # 前進で負
    py = 0  # 旋回は簡略化
    px = 0  # 上下移動なし
    return px, py, pz, vz

# 疑似乱数生成
def pseudo_random_angle(angle):
    watch = StopWatch()
    dist = get_distance()
    seed = (watch.time() + dist) % angle - angle // 2
    return seed

# 後退と旋回
def retreat(angle):
    run_at(-DEFAULT_SPEED)
    wait(1000)
    turn(pseudo_random_angle(angle))

# 距離を測りながら前進・方向転換
def run_mission_loop(ax_nominal, ay_nominal, az_nominal_moving):
    global px, py, pz, heading, last_left_angle, last_right_angle, vz
    px, py, pz, vz = 0, 0, 0, 0
    heading = 0
    last_left_angle = motor_a.angle()
    last_right_angle = motor_b.angle()
    num_anomaly = 0
    ignore_collision = True
    watch = StopWatch()  # 単一インスタンス
    start_time = watch.time()
    while True:
        try:
            run_at()
            # 開始500msは衝突検知を無視
            current_time = watch.time()
            if ignore_collision and (current_time - start_time) > 500:
                ignore_collision = False

            if get_distance() < DISTANCE_THRESHOLD:
                print("障害物検知！")
                retreat(90)
                num_anomaly += 1
                if num_anomaly == 2:
                    num_anomaly = 0
                    retreat(360)
                    px, py, pz, vz = 0, 0, 0, 0
                    last_left_angle = motor_a.angle()
                    last_right_angle = motor_b.angle()
                    ignore_collision = True
                    start_time = watch.time()

            # オドメトリ更新
            px, py, pz, vz = update_odometry()
            print("位置: px =", px, "py =", py, "pz =", pz)

            # 衝突検知
            if not ignore_collision:
                ax, ay, az = get_imu_accel_avg()
                x_diff = abs(ax - ax_nominal)
                y_diff = abs(ay - ay_nominal)
                z_diff = abs(az - az_nominal_moving)
                if y_diff > COLLISION_THRESHOLD_Y or z_diff > COLLISION_THRESHOLD_Z:
                    print("衝突検知！ X差 =", x_diff, "Y差 =", y_diff, "Z差 =", z_diff,
                          "vz =", vz, "位置: px =", px, "py =", py, "pz =", pz)
                    retreat(90)
                    num_anomaly += 1
                    if num_anomaly == 2:
                        num_anomaly = 0
                        retreat(360)
                        px, py, pz, vz = 0, 0, 0, 0
                        last_left_angle = motor_a.angle()
                        last_right_angle = motor_b.angle()
                        ignore_collision = True
                        start_time = watch.time()

            wait(100)
        except Exception as e:
            print("エラー発生:", str(e))
            wait(1000)
            run_at(0)

# メインループ
def main():
    ax_nominal, ay_nominal, _ = calibrate_imu()
    az_nominal_moving = calibrate_imu_moving()
    run_mission_loop(ax_nominal, ay_nominal, az_nominal_moving)

if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        print("プログラム終了")
        run_at(0)