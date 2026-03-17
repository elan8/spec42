/*
 * Surveillance Quadrotor Drone - SysML v2 model.
 * Reduced real-world fixture for IBD regressions.
 */
package SurveillanceDrone {
    attribute def Real;
    attribute def MassValue;
    attribute def TimeValue;
    attribute def DistanceValue;

    port def MotorCommandPort {
        in throttleCommand : Real;
    }

    port def GimbalCommandPort {
        in panAngle : Real;
        in tiltAngle : Real;
        in mode : String;
    }

    port def CameraControlPort {
        in recordCommand : String;
        in zoomCommand : Real;
        out videoStream : String;
    }

    port def TelemetryPort {
        out position : String;
        out attitude : String;
        out batteryStatus : String;
        out status : String;
    }

    port def PowerPort {
        out voltage : Real;
        out current : Real;
    }

    port def SensorDataPort {
        out position : String;
        out velocity : String;
        out attitude : String;
        out altitude : Real;
    }

    port def RCCommandPort {
        in rcCommands : String;
        in modeChanges : String;
    }

    part def Airframe {
        attribute wheelbase : Real;
        attribute weight : MassValue;
        attribute maxPayloadCapacity : MassValue;
        attribute foldableArms : String;
        attribute ipRating : String;
    }

    part def PropulsionUnit {
        attribute thrust : Real;
        attribute hoverThrottle : Real;
        attribute maxCurrent : Real;
        port cmd : ~MotorCommandPort;
        port pwr : ~PowerPort;
    }

    part def Propulsion {
        part propulsionUnit1 : PropulsionUnit;
        part propulsionUnit2 : PropulsionUnit;
        part propulsionUnit3 : PropulsionUnit;
        part propulsionUnit4 : PropulsionUnit;
        attribute thrustTotal : Real;
    }

    part def FlightController {
        attribute updateRate : Real;
        attribute supportedModes : String;
        port motorCmd : ~MotorCommandPort;
        port gimbalCmd : ~GimbalCommandPort;
        port telemetryOut : TelemetryPort;
        port rcIn : ~RCCommandPort;
        port sensorIn : ~SensorDataPort;
        port pwr : ~PowerPort;
    }

    part def IMU {
        attribute sampleRate : Real;
        port dataOut : SensorDataPort;
        port pwr : ~PowerPort;
    }

    part def Barometer {
        port altitudeOut : SensorDataPort;
        port pwr : ~PowerPort;
    }

    part def GNSSReceiver {
        port positionOut : SensorDataPort;
        port pwr : ~PowerPort;
    }

    part def FlightControlAndSensing {
        part flightController : FlightController;
        part imu : IMU;
        part barometer : Barometer;
        part gnss : GNSSReceiver;
        connect imu.dataOut to flightController.sensorIn;
        connect barometer.altitudeOut to flightController.sensorIn;
        connect gnss.positionOut to flightController.sensorIn;
    }

    part def Gimbal {
        attribute panRange : Real;
        attribute tiltRange : Real;
        port cmd : ~GimbalCommandPort;
        port cameraCtrl : CameraControlPort;
        port pwr : ~PowerPort;
    }

    part def CameraPayload {
        attribute resolution : String;
        attribute frameRate : Real;
        port videoOut : TelemetryPort;
        port control : ~CameraControlPort;
        port pwr : ~PowerPort;
    }

    part def Communication {
        attribute range : DistanceValue;
        attribute latency : TimeValue;
        port c2In : RCCommandPort;
        port telemetryIn : ~TelemetryPort;
        port videoIn : ~TelemetryPort;
        port pwr : ~PowerPort;
    }

    part def Battery {
        attribute nominalVoltage : Real;
        attribute capacity : Real;
        port pwrOut : PowerPort;
    }

    part def PowerDistribution {
        port mainPower : ~PowerPort;
        port regulated5V : PowerPort;
        port regulated12V : PowerPort;
    }

    part def Power {
        part battery : Battery;
        part distribution : PowerDistribution;
        attribute flightTimeNominal : TimeValue;
    }

    part def SurveillanceQuadrotorDrone {
        part airframe : Airframe;
        part propulsion : Propulsion;
        part flightControl : FlightControlAndSensing;
        part gimbal : Gimbal;
        part cameraPayload : CameraPayload;
        part communication : Communication;
        part power : Power;

        connect flightControl.flightController.motorCmd to propulsion.propulsionUnit1.cmd;
        connect flightControl.flightController.motorCmd to propulsion.propulsionUnit2.cmd;
        connect flightControl.flightController.motorCmd to propulsion.propulsionUnit3.cmd;
        connect flightControl.flightController.motorCmd to propulsion.propulsionUnit4.cmd;
        connect flightControl.flightController.gimbalCmd to gimbal.cmd;
        connect gimbal.cameraCtrl to cameraPayload.control;
        connect cameraPayload.videoOut to communication.videoIn;
        connect flightControl.flightController.telemetryOut to communication.telemetryIn;
        connect power.battery.pwrOut to power.distribution.mainPower;
        connect power.distribution.regulated5V to flightControl.flightController.pwr;
        connect power.distribution.regulated5V to gimbal.pwr;
        connect power.distribution.regulated5V to cameraPayload.pwr;
        connect power.distribution.regulated5V to communication.pwr;
        connect power.distribution.regulated5V to propulsion.propulsionUnit1.pwr;
        connect power.distribution.regulated5V to propulsion.propulsionUnit2.pwr;
        connect power.distribution.regulated5V to propulsion.propulsionUnit3.pwr;
        connect power.distribution.regulated5V to propulsion.propulsionUnit4.pwr;
    }
}
