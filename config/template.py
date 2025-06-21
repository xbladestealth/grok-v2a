from pybricks.hubs import MoveHub
from pybricks.pupdevices import Motor, ColorDistanceSensor
from pybricks.parameters import Button, Color, Direction, Port, Side, Stop
from pybricks.robotics import DriveBase
from pybricks.tools import wait, StopWatch

hub = MoveHub()

# モーターとセンサー設定
motor_a = Motor(Port.A, positive_direction=Direction.COUNTERCLOCKWISE)
motor_b = Motor(Port.B, positive_direction=Direction.CLOCKWISE)
sensor = ColorDistanceSensor(Port.C) # 距離センサー

# 距離を測りながら前進
while True:
    distance = sensor.distance() # 距離を測る（mm）
    if distance < 40: # 10cm以内に障害物 
        # 後退 
        motor_a.run(-360)
        motor_b.run(-360)
        wait(1000)
        motor_a.stop()
        motor_b.stop()
    else: # 前進
        motor_a.run(360)
        motor_b.run(360)
    wait(100) # 0.1秒ごとにチェック