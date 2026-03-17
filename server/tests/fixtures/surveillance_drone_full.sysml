/*
 * Surveillance Quadrotor Drone - SysML v2 model.
 * Derived from drone/README.md: §4 structure, §5 requirements, §3.2 use cases, §6 interfaces.
 * Propulsion modeled as 4x PropulsionUnit.
 */
package SurveillanceDrone {
	doc /* Root package for the surveillance quadrotor drone example. */

	// ========== Value types (simplified) ==========
	attribute def Real;
	attribute def MassValue;
	attribute def TimeValue;
	attribute def DistanceValue;
	attribute def SpeedValue;

	// ========== Port definitions (interfaces) ==========
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

	// ========== Part definitions - Subsystems ==========
	part def Airframe {
		doc /* Structural support, attachment points, quad-X or H layout. */
		attribute wheelbase : Real;
		attribute weight : MassValue;
		attribute maxPayloadCapacity : MassValue;
		attribute foldableArms : String;
		attribute ipRating : String;
	}

	part def PropulsionUnit {
		doc /* One motor + ESC + propeller; BLDC outrunner, PWM/DShot input. */
		attribute thrust : Real;
		attribute hoverThrottle : Real;
		attribute maxCurrent : Real;
		port cmd : ~MotorCommandPort;
		port pwr : ~PowerPort;
	}

	part def Propulsion {
		doc /* Four propulsion units for quadrotor lift and thrust. */
		part propulsionUnit1 : PropulsionUnit;
		part propulsionUnit2 : PropulsionUnit;
		part propulsionUnit3 : PropulsionUnit;
		part propulsionUnit4 : PropulsionUnit;
		attribute thrustTotal : Real;
	}

	part def FlightController {
		doc /* Main processor: attitude/rate control, mixing, flight modes. */
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
		doc /* FC + IMU, baro, GNSS for stabilization and navigation. */
		part flightController : FlightController;
		part imu : IMU;
		part barometer : Barometer;
		part gnss : GNSSReceiver;
		connect imu.dataOut to flightController.sensorIn;
		connect barometer.altitudeOut to flightController.sensorIn;
		connect gnss.positionOut to flightController.sensorIn;
		attribute geofenceBehavior : String;
		attribute failsafeLogic : String;
	}

	part def Gimbal {
		doc /* Mechanically stabilized platform; pan/tilt, optionally roll. */
		attribute panRange : Real;
		attribute tiltRange : Real;
		attribute maxAngularRate : Real;
		attribute payloadCapacity : MassValue;
		port cmd : ~GimbalCommandPort;
		port cameraCtrl : CameraControlPort;
		port pwr : ~PowerPort;
	}

	part def CameraPayload {
		doc /* Visible and/or thermal camera; video and stills. */
		attribute resolution : String;
		attribute frameRate : Real;
		attribute fov : Real;
		attribute zoomRange : Real;
		port videoOut : TelemetryPort;
		port control : ~CameraControlPort;
		port pwr : ~PowerPort;
	}

	part def Communication {
		doc /* C2 link, telemetry, video downlink. */
		attribute range : DistanceValue;
		attribute latency : TimeValue;
		attribute bandwidth : Real;
		port c2In : RCCommandPort;
		port telemetryIn : ~TelemetryPort;
		port videoIn : ~TelemetryPort;
		port pwr : ~PowerPort;
	}

	part def Battery {
		attribute nominalVoltage : Real;
		attribute capacity : Real;
		attribute maxDischargeRate : Real;
		attribute weight : MassValue;
		port pwrOut : PowerPort;
	}

	part def PowerDistribution {
		port mainPower : ~PowerPort;
		port regulated5V : PowerPort;
		port regulated12V : PowerPort;
	}

	part def Power {
		doc /* Battery + PDB; supplies all onboard systems. */
		part battery : Battery;
		part distribution : PowerDistribution;
		attribute flightTimeNominal : TimeValue;
	}

	// ========== Top-level system ==========
	part def SurveillanceQuadrotorDrone {
		doc /*
		 * Surveillance quadrotor UAV with gimbal-mounted cameras.
		 * Structure per README §4; interfaces per §6.
		 */
		part airframe : Airframe;
		part propulsion : Propulsion;
		part flightControl : FlightControlAndSensing;
		part gimbal : Gimbal;
		part cameraPayload : CameraPayload;
		part communication : Communication;
		part power : Power;

		// Connections per README §6
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

	// ========== Requirements (README §5) ==========
	requirement def EnduranceReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Minimum flight time at nominal hover with standard payload. */
		require constraint { doc /* flightTime >= 25 min. */ }
	}

	requirement def MaxAltitudeAGLReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Max altitude AGL per regulation and mission (e.g. 120 m). */
		require constraint { doc /* altitudeAGL <= 120 m. */ }
	}

	requirement def WindToleranceReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Operate in winds up to specified speed (e.g. 10 m/s). */
		require constraint { doc /* windSpeed <= 10 m/s. */ }
	}

	requirement def VideoLatencyReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Video end-to-end latency < 300 ms. */
		require constraint { doc /* videoLatency < 300 ms. */ }
	}

	requirement def FailsafeReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* RTL or land on C2 loss, low battery, critical sensor failure. */
		require constraint { doc /* On C2 loss or low battery or critical sensor failure: execute RTL or land. */ }
	}

	requirement def GeofencingReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Respect no-fly zones and altitude limits. */
		require constraint { doc /* Respect geofence and altitude limits. */ }
	}

	requirement def RemoteIDReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Remote ID or equivalent where required. */
		require constraint { doc /* Provide Remote ID where required. */ }
	}

	requirement def GimbalPanTiltReq {
		subject gimbal : Gimbal;
		doc /* 360° pan or sufficient range; tilt e.g. -90° to +30°. */
		require constraint { doc /* panRange >= 360; tiltRange covers -90 to +30. */ }
	}

	requirement def ControlModesReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Manual, attitude, GPS hold, waypoint, orbit, RTL, optionally follow. */
		require constraint { doc /* Support manual, attitude, GPS hold, waypoint, orbit, RTL, follow. */ }
	}

	requirement def OperatingTempReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Operating temperature e.g. -10 °C to +45 °C. */
		require constraint { doc /* Operate in -10 °C to +45 °C. */ }
	}

	// Satisfy requirements by the drone (usage in context)
	part droneInstance : SurveillanceQuadrotorDrone;
	satisfy EnduranceReq by droneInstance;
	satisfy MaxAltitudeAGLReq by droneInstance;
	satisfy WindToleranceReq by droneInstance;
	satisfy VideoLatencyReq by droneInstance;
	satisfy FailsafeReq by droneInstance;
	satisfy GeofencingReq by droneInstance;
	satisfy RemoteIDReq by droneInstance;
	satisfy GimbalPanTiltReq by droneInstance.gimbal;
	satisfy ControlModesReq by droneInstance;
	satisfy OperatingTempReq by droneInstance;

	// ========== Use cases (README §3.2) ==========
	item def Operator;

	use case def PatrolOverwatch {
		subject drone : SurveillanceQuadrotorDrone;
		actor pilot : Operator;
		doc /* Predefined or manual route; continuous video to GCS. */
		objective { doc /* Provide continuous video of route to GCS. */ }
	}

	use case def PointOfInterestOrbit {
		subject drone : SurveillanceQuadrotorDrone;
		actor pilot : Operator;
		doc /* Circle fixed point with camera on center; adjustable radius and altitude. */
		objective { doc /* Maintain orbit around POI with camera on target. */ }
	}

	use case def FollowTarget {
		subject drone : SurveillanceQuadrotorDrone;
		actor pilot : Operator;
		doc /* Track moving object with gimbal while maintaining safe flight. */
		objective { doc /* Keep target in frame while flying safely. */ }
	}

	use case def Inspection {
		subject drone : SurveillanceQuadrotorDrone;
		actor pilot : Operator;
		doc /* Structured path (e.g. grid) for infrastructure/area inspection. */
		objective { doc /* Capture high-resolution stills and video for inspection. */ }
	}

	use case def Search {
		subject drone : SurveillanceQuadrotorDrone;
		actor pilot : Operator;
		doc /* Area search with thermal camera; loiter and zoom as needed. */
		objective { doc /* Locate targets using thermal and visible imagery. */ }
	}

	use case def ReturnToLaunchLanding {
		subject drone : SurveillanceQuadrotorDrone;
		actor pilot : Operator;
		doc /* Automated or assisted return and landing with payload secured. */
		objective { doc /* Return to launch point and land safely. */ }
	}

	// ========== Behavior - Actions ==========
	action def ExecutePatrol {
		in route : String;
		out status : String;
		doc /* Execute patrol/overwatch mission along route. */
	}

	action def ExecuteOrbit {
		in center : String;
		in radius : Real;
		in altitude : Real;
		out status : String;
		doc /* Orbit around point of interest. */
	}

	action def ControlGimbal {
		in panAngle : Real;
		in tiltAngle : Real;
		in mode : String;
		doc /* Command gimbal pointing and mode. */
	}

	action def CaptureVideo {
		out videoStream : String;
		doc /* Capture and output video stream. */
	}

	// Parts perform actions; allocations use named action usages (source/target must be usages per spec 7.15).
	part def SurveillanceQuadrotorDroneWithBehavior :> SurveillanceQuadrotorDrone {
		perform action executePatrol : ExecutePatrol;
		perform action executeOrbit : ExecuteOrbit;
		perform action controlGimbal : ControlGimbal;
		perform action captureVideo : CaptureVideo;
		allocate executePatrol to flightControl;
		allocate executeOrbit to flightControl;
		allocate controlGimbal to gimbal;
		allocate captureVideo to cameraPayload;
	}

	// ========== State machine - Flight modes ==========
	state def Manual;
	state def AttitudeHold;
	state def GPSHold;
	state def Waypoint;
	state def Orbit;
	state def Follow;
	state def RTL;
	state def Landing;

	state def FlightModeStateMachine {
		state manual : Manual;
		state attitudeHold : AttitudeHold;
		state gpsHold : GPSHold;
		state waypoint : Waypoint;
		state orbit : Orbit;
		state follow : Follow;
		state rtl : RTL;
		state landing : Landing;
	}

	// ========== Parametrics ==========
	constraint def TotalThrustMargin {
		doc /* Total thrust >= weight with margin for hover and maneuver. */
		in totalThrust : Real;
		in totalWeight : Real;
		in margin : Real;
		totalThrust >= totalWeight * margin
	}

	calc def FlightTimeEstimate {
		doc /* Flight time from capacity and current draw. */
		in capacity : Real;
		in currentDraw : Real;
		return flightTime : Real;
		capacity / currentDraw
	}

}
