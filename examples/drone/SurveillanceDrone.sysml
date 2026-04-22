/*
 * Surveillance Quadrotor Drone - learner-focused SysML v2 worked example.
 * This model is broader and less formalized than the smaller examples.
 * Propulsion is modeled as 4x PropulsionUnit for readability.
 */
package SurveillanceDrone {
	doc /* Root package for a broad surveillance-drone teaching example. */

	// ========== Standard library imports ==========
	private import ScalarValues::Real;
	private import ScalarValues::String;
	private import ISQBase::MassValue;
	private import ISQBase::LengthValue;
	private import ISQBase::DurationValue;

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

	// ========== Part definitions - subsystems ==========
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
		port cmd : MotorCommandPort;
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
		exhibit state flightMode : FlightModeStateMachine;
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
		port cmd : GimbalCommandPort;
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
		attribute range : LengthValue;
		attribute latency : DurationValue;
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
		doc /* Battery + distribution; supplies all onboard systems. */
		part battery : Battery;
		part distribution : PowerDistribution;
		attribute flightTimeNominal : DurationValue;
	}

	// ========== Top-level system ==========
	part def SurveillanceQuadrotorDrone {
		doc /* Surveillance quadrotor UAV with gimbal-mounted cameras. */
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
		connect communication.c2In to flightControl.flightController.rcIn;
		connect power.battery.pwrOut to power.distribution.mainPower;
		connect power.distribution.regulated5V to flightControl.flightController.pwr;
		connect power.distribution.regulated5V to flightControl.imu.pwr;
		connect power.distribution.regulated5V to flightControl.barometer.pwr;
		connect power.distribution.regulated5V to flightControl.gnss.pwr;
		connect power.distribution.regulated5V to gimbal.pwr;
		connect power.distribution.regulated5V to cameraPayload.pwr;
		connect power.distribution.regulated5V to communication.pwr;
		connect power.distribution.regulated12V to propulsion.propulsionUnit1.pwr;
		connect power.distribution.regulated12V to propulsion.propulsionUnit2.pwr;
		connect power.distribution.regulated12V to propulsion.propulsionUnit3.pwr;
		connect power.distribution.regulated12V to propulsion.propulsionUnit4.pwr;
	}

	// ========== Requirements ==========
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
		subject communication : Communication;
		doc /* Video end-to-end latency < 300 ms. */
		require constraint { doc /* videoLatency < 300 ms. */ }
	}

	requirement def FailsafeReq {
		subject controller : FlightController;
		doc /* RTL or land on C2 loss, low battery, critical sensor failure. */
		require constraint { doc /* On C2 loss or low battery or critical sensor failure: execute RTL or land. */ }
	}

	requirement def GeofencingReq {
		subject flightControl : FlightControlAndSensing;
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
		doc /* 360 deg pan or sufficient range; tilt e.g. -90 deg to +30 deg. */
		require constraint { doc /* panRange >= 360; tiltRange covers -90 to +30. */ }
	}

	requirement def ControlModesReq {
		subject controller : FlightController;
		doc /* Manual, attitude, GPS hold, waypoint, orbit, RTL, optionally follow. */
		require constraint { doc /* Support manual, attitude, GPS hold, waypoint, orbit, RTL, follow. */ }
	}

	requirement def OperatingTempReq {
		subject drone : SurveillanceQuadrotorDrone;
		doc /* Operating temperature e.g. -10 degC to +45 degC. */
		require constraint { doc /* Operate in -10 degC to +45 degC. */ }
	}

	part droneInstance : SurveillanceQuadrotorDrone;
	satisfy EnduranceReq by droneInstance;
	satisfy MaxAltitudeAGLReq by droneInstance;
	satisfy WindToleranceReq by droneInstance;
	satisfy VideoLatencyReq by droneInstance.communication;
	satisfy FailsafeReq by droneInstance.flightControl.flightController;
	satisfy GeofencingReq by droneInstance.flightControl;
	satisfy RemoteIDReq by droneInstance;
	satisfy GimbalPanTiltReq by droneInstance.gimbal;
	satisfy ControlModesReq by droneInstance.flightControl.flightController;
	satisfy OperatingTempReq by droneInstance;

	// ========== Use cases ==========
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

	// ========== Behavior - mission actions ==========
	action def ExecutePatrol {
		in route : String;
		out status : String;
		doc /* High-level patrol mission action for route-based surveillance. */
		action validateRoute { out routeReady : Boolean; }
		action armForMission { out armed : Boolean; }
		action takeOff { out airborne : Boolean; }
		action flyPatrolRoute { out routeComplete : Boolean; }
		action captureSurveyPass { out imageryCaptured : Boolean; }
		action reportMissionStatus { out missionStatus : String; }
		first validateRoute then armForMission;
		first armForMission then takeOff;
		first takeOff then flyPatrolRoute;
		first flyPatrolRoute then captureSurveyPass;
		first captureSurveyPass then reportMissionStatus;
	}

	action def ExecuteOrbit {
		in center : String;
		in radius : Real;
		in altitude : Real;
		out status : String;
		doc /* High-level orbit mission action around a point of interest. */
		action validateOrbitPoint { out orbitValid : Boolean; }
		action climbToOrbitAltitude { out altitudeReached : Boolean; }
		action enterOrbitPattern { out orbitEstablished : Boolean; }
		action stabilizeOrbitTrack { out orbitStable : Boolean; }
		action reportOrbitStatus { out orbitStatus : String; }
		first validateOrbitPoint then climbToOrbitAltitude;
		first climbToOrbitAltitude then enterOrbitPattern;
		first enterOrbitPattern then stabilizeOrbitTrack;
		first stabilizeOrbitTrack then reportOrbitStatus;
	}

	action def ControlGimbal {
		in panAngle : Real;
		in tiltAngle : Real;
		in mode : String;
		doc /* High-level command for gimbal pointing and mode selection. */
		action interpretModeCommand { out commandReady : Boolean; }
		action slewPanAxis { out panSettled : Boolean; }
		action slewTiltAxis { out tiltSettled : Boolean; }
		action stabilizeLineOfSight { out sightStable : Boolean; }
		action confirmGimbalAim { out aimConfirmed : Boolean; }
		first interpretModeCommand then slewPanAxis;
		first slewPanAxis then slewTiltAxis;
		first slewTiltAxis then stabilizeLineOfSight;
		first stabilizeLineOfSight then confirmGimbalAim;
	}

	action def CaptureVideo {
		out videoStream : String;
		doc /* High-level action for producing a surveillance video stream. */
		action powerImager { out imagerPowered : Boolean; }
		action configureEncoder { out encoderReady : Boolean; }
		action startCapture { out captureStarted : Boolean; }
		action streamVideoFrames { out encodedFrames : String; }
		action publishVideoStream { out streamPublished : String; }
		first powerImager then configureEncoder;
		first configureEncoder then startCapture;
		first startCapture then streamVideoFrames;
		first streamVideoFrames then publishVideoStream;
	}

	// ========== State machine - flight modes ==========
	state def Manual {
		doc /* Direct pilot control with minimal stabilization assumptions. */
	}

	state def AttitudeHold {
		doc /* Stabilized flight while pilot commands attitude and thrust. */
	}

	state def GPSHold {
		doc /* Position-hold mode using GNSS and attitude stabilization. */
	}

	state def Waypoint {
		doc /* Autonomous waypoint-following mission mode. */
	}

	state def Orbit {
		doc /* Automated orbit around a point of interest. */
	}

	state def Follow {
		doc /* Tracking mode for a moving subject or designated target. */
	}

	state def RTL {
		doc /* Return-to-launch recovery mode. */
	}

	state def Landing {
		doc /* Final descent and touchdown sequence. */
	}

	item def ManualSelected;
	item def AttitudeHoldSelected;
	item def GPSHoldSelected;
	item def WaypointSelected;
	item def OrbitSelected;
	item def FollowSelected;
	item def RTLCommanded;
	item def LandingCommanded;
	item def DisarmCommanded;
	item def C2LinkLost;
	item def LowBattery;
	item def CriticalSensorFailure;
	item def Recovered;

	state def FlightModeStateMachine {
		doc /* Flight-controller mode set with operator and failsafe transitions. */
		state manual : Manual;
		state attitudeHold : AttitudeHold;
		state gpsHold : GPSHold;
		state waypoint : Waypoint;
		state orbit : Orbit;
		state follow : Follow;
		state rtl : RTL;
		state landing : Landing;

		entry;
		then manual;

		transition to_attitude_from_manual first manual accept AttitudeHoldSelected then attitudeHold;
		transition to_gpshold_from_manual first manual accept GPSHoldSelected then gpsHold;
		transition to_waypoint_from_manual first manual accept WaypointSelected then waypoint;
		transition to_orbit_from_manual first manual accept OrbitSelected then orbit;
		transition to_follow_from_manual first manual accept FollowSelected then follow;
		transition to_rtl_from_manual first manual accept RTLCommanded then rtl;
		transition to_landing_from_manual first manual accept LandingCommanded then landing;

		transition to_manual_from_attitude first attitudeHold accept ManualSelected then manual;
		transition to_gpshold_from_attitude first attitudeHold accept GPSHoldSelected then gpsHold;
		transition to_waypoint_from_attitude first attitudeHold accept WaypointSelected then waypoint;
		transition to_orbit_from_attitude first attitudeHold accept OrbitSelected then orbit;
		transition to_follow_from_attitude first attitudeHold accept FollowSelected then follow;
		transition to_rtl_from_attitude first attitudeHold accept RTLCommanded then rtl;
		transition to_landing_from_attitude first attitudeHold accept LandingCommanded then landing;

		transition to_manual_from_gpshold first gpsHold accept ManualSelected then manual;
		transition to_attitude_from_gpshold first gpsHold accept AttitudeHoldSelected then attitudeHold;
		transition to_waypoint_from_gpshold first gpsHold accept WaypointSelected then waypoint;
		transition to_orbit_from_gpshold first gpsHold accept OrbitSelected then orbit;
		transition to_follow_from_gpshold first gpsHold accept FollowSelected then follow;
		transition to_rtl_from_gpshold first gpsHold accept RTLCommanded then rtl;
		transition to_landing_from_gpshold first gpsHold accept LandingCommanded then landing;

		transition to_manual_from_waypoint first waypoint accept ManualSelected then manual;
		transition to_attitude_from_waypoint first waypoint accept AttitudeHoldSelected then attitudeHold;
		transition to_gpshold_from_waypoint first waypoint accept GPSHoldSelected then gpsHold;
		transition to_orbit_from_waypoint first waypoint accept OrbitSelected then orbit;
		transition to_follow_from_waypoint first waypoint accept FollowSelected then follow;
		transition to_rtl_from_waypoint first waypoint accept RTLCommanded then rtl;
		transition to_landing_from_waypoint first waypoint accept LandingCommanded then landing;

		transition to_manual_from_orbit first orbit accept ManualSelected then manual;
		transition to_attitude_from_orbit first orbit accept AttitudeHoldSelected then attitudeHold;
		transition to_gpshold_from_orbit first orbit accept GPSHoldSelected then gpsHold;
		transition to_waypoint_from_orbit first orbit accept WaypointSelected then waypoint;
		transition to_follow_from_orbit first orbit accept FollowSelected then follow;
		transition to_rtl_from_orbit first orbit accept RTLCommanded then rtl;
		transition to_landing_from_orbit first orbit accept LandingCommanded then landing;

		transition to_manual_from_follow first follow accept ManualSelected then manual;
		transition to_attitude_from_follow first follow accept AttitudeHoldSelected then attitudeHold;
		transition to_gpshold_from_follow first follow accept GPSHoldSelected then gpsHold;
		transition to_waypoint_from_follow first follow accept WaypointSelected then waypoint;
		transition to_orbit_from_follow first follow accept OrbitSelected then orbit;
		transition to_rtl_from_follow first follow accept RTLCommanded then rtl;
		transition to_landing_from_follow first follow accept LandingCommanded then landing;

		transition failsafe_rtl_c2loss first manual accept C2LinkLost then rtl;
		transition failsafe_rtl_c2loss_ah first attitudeHold accept C2LinkLost then rtl;
		transition failsafe_rtl_c2loss_gps first gpsHold accept C2LinkLost then rtl;
		transition failsafe_rtl_c2loss_wp first waypoint accept C2LinkLost then rtl;
		transition failsafe_rtl_c2loss_orbit first orbit accept C2LinkLost then rtl;
		transition failsafe_rtl_c2loss_follow first follow accept C2LinkLost then rtl;

		transition failsafe_rtl_lowbat first manual accept LowBattery then rtl;
		transition failsafe_rtl_lowbat_ah first attitudeHold accept LowBattery then rtl;
		transition failsafe_rtl_lowbat_gps first gpsHold accept LowBattery then rtl;
		transition failsafe_rtl_lowbat_wp first waypoint accept LowBattery then rtl;
		transition failsafe_rtl_lowbat_orbit first orbit accept LowBattery then rtl;
		transition failsafe_rtl_lowbat_follow first follow accept LowBattery then rtl;

		transition failsafe_rtl_sensor first manual accept CriticalSensorFailure then rtl;
		transition failsafe_rtl_sensor_ah first attitudeHold accept CriticalSensorFailure then rtl;
		transition failsafe_rtl_sensor_gps first gpsHold accept CriticalSensorFailure then rtl;
		transition failsafe_rtl_sensor_wp first waypoint accept CriticalSensorFailure then rtl;
		transition failsafe_rtl_sensor_orbit first orbit accept CriticalSensorFailure then rtl;
		transition failsafe_rtl_sensor_follow first follow accept CriticalSensorFailure then rtl;

		transition rtl_to_landing first rtl accept LandingCommanded then landing;
		transition rtl_recover_to_gpshold first rtl accept Recovered then gpsHold;
		transition landing_to_manual_disarm first landing accept DisarmCommanded then manual;
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


