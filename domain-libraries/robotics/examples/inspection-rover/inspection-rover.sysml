package InspectionRoverExample {
    import RoboticsCore::*;
    import RobotRuntime::*;
    import RobotStructure::*;
    import Ros2Overlay::*;

    // Runtime responsibility map:
    // - operator_console_node: publishes mission GPS targets from user/operator intent.
    // - mission_supervisor_node: translates GPS mission targets into map-frame navigation goals
    //   and executes inspection workflow once goal arrival is confirmed.
    // - localization_node: estimates current robot pose and publishes odometry.
    // - navigation_node: plans/controls toward goal pose and emits velocity + goal reached events.
    // - motion_controller_node: converts velocity commands into motor-level actuation.

    part def InspectionRover :> RobotPlatform {
        attribute :>> name = "inspection_rover";
        attribute :>> criticality = high;
    }

    part def RoverSystem :> InspectionRover {
        part runtime : RoverRuntimeArchitecture;
        part deployment : RoverDeployment;
        part traceability : RoverTraceability;
    }

    part def RoverRuntimeArchitecture {
        part operatorRuntime : OperatorConsoleNode;
        part missionRuntime : MissionSupervisorNode;
        part lidarRuntime : LidarNode;
        part localizationRuntime : LocalizationNode;
        part navigationRuntime : NavigationNode;
        part motionRuntime : MotionControllerNode;

        connect operatorRuntime.gpsTargetPubPort to missionRuntime.gpsTargetSubPort;
        connect missionRuntime.goalPubPort to navigationRuntime.goalSubPort;
        connect lidarRuntime.scanPubPort to localizationRuntime.scanSubPort;
        connect localizationRuntime.odomPubPort to navigationRuntime.odomSubPort;
        connect navigationRuntime.cmdVelPubPort to motionRuntime.cmdVelSubPort;
        connect navigationRuntime.goalReachedPubPort to missionRuntime.inspectTriggerSubPort;
    }

    part def RoverRuntimeNode :> RosLifecycleNode {
        attribute :>> packageRef = "inspection_rover_bringup";
        attribute :>> namespaceRef = "/inspection_rover";
        attribute :>> launchGroupName = "core_runtime";
    }

    part def RoverCoreRuntimeNode :> RosRuntimeComponent {
        attribute :>> packageRef = "inspection_rover_bringup";
        attribute :>> namespaceRef = "/inspection_rover";
        attribute :>> launchGroupName = "core_runtime";
    }

    part def LidarNode :> RoverRuntimeNode {
        attribute :>> name = "lidar_node";
        attribute :>> executableName = "lidar_node";

        port scanPubPort : RosPublishPort {
            attribute :>> name = "scan_pub";
            attribute :>> ownerNodeRef = "lidar_node";
            attribute :>> topicRef = "/scan";
            attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
            attribute :>> qosProfileRef = "sensor_data";
        }
    }

    part def LocalizationNode :> RoverRuntimeNode {
        attribute :>> name = "localization_node";
        attribute :>> executableName = "localization_node";

        port scanSubPort : RosSubscribePort {
            attribute :>> name = "scan_sub";
            attribute :>> ownerNodeRef = "localization_node";
            attribute :>> topicRef = "/scan";
            attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
            attribute :>> qosProfileRef = "sensor_data";
        }

        port odomPubPort : RosPublishPort {
            attribute :>> name = "odom_pub";
            attribute :>> ownerNodeRef = "localization_node";
            attribute :>> topicRef = "/odom";
            attribute :>> messageTypeRef = "nav_msgs/Odometry";
            attribute :>> qosProfileRef = "sensor_data";
        }
    }

    part def MissionSupervisorNode :> RoverCoreRuntimeNode {
        attribute :>> name = "mission_supervisor_node";
        attribute :>> executableName = "mission_supervisor_node";

        port gpsTargetSubPort : RosSubscribePort {
            attribute :>> name = "gps_target_sub";
            attribute :>> ownerNodeRef = "mission_supervisor_node";
            attribute :>> topicRef = "/mission_target_gps";
            attribute :>> messageTypeRef = "sensor_msgs/NavSatFix";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port goalPubPort : RosPublishPort {
            attribute :>> name = "goal_pub";
            attribute :>> ownerNodeRef = "mission_supervisor_node";
            attribute :>> topicRef = "/goal_pose";
            attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port inspectTriggerSubPort : RosSubscribePort {
            attribute :>> name = "inspect_trigger_sub";
            attribute :>> ownerNodeRef = "mission_supervisor_node";
            attribute :>> topicRef = "/goal_reached";
            attribute :>> messageTypeRef = "std_msgs/Bool";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port inspectionCompletePubPort : RosPublishPort {
            attribute :>> name = "inspection_complete_pub";
            attribute :>> ownerNodeRef = "mission_supervisor_node";
            attribute :>> topicRef = "/inspection_complete";
            attribute :>> messageTypeRef = "std_msgs/Bool";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    part def OperatorConsoleNode :> RoverCoreRuntimeNode {
        attribute :>> name = "operator_console_node";
        attribute :>> executableName = "operator_console_node";

        port gpsTargetPubPort : RosPublishPort {
            attribute :>> name = "gps_target_pub";
            attribute :>> ownerNodeRef = "operator_console_node";
            attribute :>> topicRef = "/mission_target_gps";
            attribute :>> messageTypeRef = "sensor_msgs/NavSatFix";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    part def NavigationNode :> RoverRuntimeNode {
        attribute :>> name = "navigation_node";
        attribute :>> executableName = "navigation_node";

        port odomSubPort : RosSubscribePort {
            attribute :>> name = "odom_sub";
            attribute :>> ownerNodeRef = "navigation_node";
            attribute :>> topicRef = "/odom";
            attribute :>> messageTypeRef = "nav_msgs/Odometry";
            attribute :>> qosProfileRef = "sensor_data";
        }

        port goalSubPort : RosSubscribePort {
            attribute :>> name = "goal_sub";
            attribute :>> ownerNodeRef = "navigation_node";
            attribute :>> topicRef = "/goal_pose";
            attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port cmdVelPubPort : RosPublishPort {
            attribute :>> name = "cmd_vel_pub";
            attribute :>> ownerNodeRef = "navigation_node";
            attribute :>> topicRef = "/cmd_vel";
            attribute :>> messageTypeRef = "geometry_msgs/Twist";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port goalReachedPubPort : RosPublishPort {
            attribute :>> name = "goal_reached_pub";
            attribute :>> ownerNodeRef = "navigation_node";
            attribute :>> topicRef = "/goal_reached";
            attribute :>> messageTypeRef = "std_msgs/Bool";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    part def MotionControllerNode :> RoverCoreRuntimeNode {
        attribute :>> name = "motion_controller_node";
        attribute :>> executableName = "motion_controller_node";

        port cmdVelSubPort : RosSubscribePort {
            attribute :>> name = "cmd_vel_sub";
            attribute :>> ownerNodeRef = "motion_controller_node";
            attribute :>> topicRef = "/cmd_vel";
            attribute :>> messageTypeRef = "geometry_msgs/Twist";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    item def LaserScanMessage :> RosMessageType {
        attribute :>> name = "sensor_msgs/LaserScan";
    }

    item def TwistMessage :> RosMessageType {
        attribute :>> name = "geometry_msgs/Twist";
    }

    item def OdomMessage :> RosMessageType {
        attribute :>> name = "nav_msgs/Odometry";
    }

    item def GoalPoseMessage :> RosMessageType {
        attribute :>> name = "geometry_msgs/PoseStamped";
    }

    item def GpsTargetMessage :> RosMessageType {
        attribute :>> name = "sensor_msgs/NavSatFix";
    }

    item def BoolMessage :> RosMessageType {
        attribute :>> name = "std_msgs/Bool";
    }

    item def SetRuntimeModeService :> RosServiceType {
        attribute :>> name = "inspection_msgs/SetRuntimeMode";
    }

    item def NavigateToPointAction :> RosActionType {
        attribute :>> name = "inspection_msgs/NavigateToPoint";
    }

    part def ScanTopic :> RosTopic {
        attribute :>> name = "/scan";
        attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
    }

    part def CmdVelTopic :> RosTopic {
        attribute :>> name = "/cmd_vel";
        attribute :>> messageTypeRef = "geometry_msgs/Twist";
    }

    part def OdomTopic :> RosTopic {
        attribute :>> name = "/odom";
        attribute :>> messageTypeRef = "nav_msgs/Odometry";
    }

    part def GoalPoseTopic :> RosTopic {
        attribute :>> name = "/goal_pose";
        attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
    }

    part def GpsTargetTopic :> RosTopic {
        attribute :>> name = "/mission_target_gps";
        attribute :>> messageTypeRef = "sensor_msgs/NavSatFix";
    }

    part def GoalReachedTopic :> RosTopic {
        attribute :>> name = "/goal_reached";
        attribute :>> messageTypeRef = "std_msgs/Bool";
    }

    part def InspectionCompleteTopic :> RosTopic {
        attribute :>> name = "/inspection_complete";
        attribute :>> messageTypeRef = "std_msgs/Bool";
    }

    part def SensorQos :> RosQosProfile {
        attribute :>> name = "sensor_data";
        attribute :>> reliability = best_effort;
    }

    part def ControlQos :> RosQosProfile {
        attribute :>> name = "control_reliable";
        attribute :>> reliability = reliable;
    }

    part def ScanPublisher :> RosPublisher {
        attribute :>> name = "lidar_scan_pub";
        attribute :>> publisherNode = "lidar_node";
        attribute :>> topicRef = "/scan";
        attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
        attribute :>> qosProfileRef = "sensor_data";
    }

    part def ScanSubscriber :> RosSubscriber {
        attribute :>> name = "localization_scan_sub";
        attribute :>> subscriberNode = "localization_node";
        attribute :>> topicRef = "/scan";
        attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
        attribute :>> qosProfileRef = "sensor_data";
    }

    part def CmdVelPublisher :> RosPublisher {
        attribute :>> name = "navigation_cmd_vel_pub";
        attribute :>> publisherNode = "navigation_node";
        attribute :>> topicRef = "/cmd_vel";
        attribute :>> messageTypeRef = "geometry_msgs/Twist";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def OdomPublisher :> RosPublisher {
        attribute :>> name = "localization_odom_pub";
        attribute :>> publisherNode = "localization_node";
        attribute :>> topicRef = "/odom";
        attribute :>> messageTypeRef = "nav_msgs/Odometry";
        attribute :>> qosProfileRef = "sensor_data";
    }

    part def OdomSubscriber :> RosSubscriber {
        attribute :>> name = "navigation_odom_sub";
        attribute :>> subscriberNode = "navigation_node";
        attribute :>> topicRef = "/odom";
        attribute :>> messageTypeRef = "nav_msgs/Odometry";
        attribute :>> qosProfileRef = "sensor_data";
    }

    part def GoalPoseSubscriber :> RosSubscriber {
        attribute :>> name = "navigation_goal_sub";
        attribute :>> subscriberNode = "navigation_node";
        attribute :>> topicRef = "/goal_pose";
        attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def GoalPosePublisher :> RosPublisher {
        attribute :>> name = "mission_goal_pub";
        attribute :>> publisherNode = "mission_supervisor_node";
        attribute :>> topicRef = "/goal_pose";
        attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def GpsTargetPublisher :> RosPublisher {
        attribute :>> name = "operator_gps_target_pub";
        attribute :>> publisherNode = "operator_console_node";
        attribute :>> topicRef = "/mission_target_gps";
        attribute :>> messageTypeRef = "sensor_msgs/NavSatFix";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def GpsTargetSubscriber :> RosSubscriber {
        attribute :>> name = "mission_gps_target_sub";
        attribute :>> subscriberNode = "mission_supervisor_node";
        attribute :>> topicRef = "/mission_target_gps";
        attribute :>> messageTypeRef = "sensor_msgs/NavSatFix";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def GoalReachedPublisher :> RosPublisher {
        attribute :>> name = "navigation_goal_reached_pub";
        attribute :>> publisherNode = "navigation_node";
        attribute :>> topicRef = "/goal_reached";
        attribute :>> messageTypeRef = "std_msgs/Bool";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def GoalReachedSubscriber :> RosSubscriber {
        attribute :>> name = "mission_goal_reached_sub";
        attribute :>> subscriberNode = "mission_supervisor_node";
        attribute :>> topicRef = "/goal_reached";
        attribute :>> messageTypeRef = "std_msgs/Bool";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def InspectionCompletePublisher :> RosPublisher {
        attribute :>> name = "mission_inspection_complete_pub";
        attribute :>> publisherNode = "mission_supervisor_node";
        attribute :>> topicRef = "/inspection_complete";
        attribute :>> messageTypeRef = "std_msgs/Bool";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def SetRuntimeModeServer :> RosService {
        attribute :>> name = "set_runtime_mode";
        attribute :>> serviceTypeRef = "inspection_msgs/SetRuntimeMode";
    }

    part def NavigateToPointServer :> RosActionServer {
        attribute :>> name = "navigate_to_point";
        attribute :>> actionTypeRef = "inspection_msgs/NavigateToPoint";
    }

    part def LocalizationRateParam :> RosParameter {
        attribute :>> name = "localization_rate_hz";
        attribute :>> ownerNodeRef = "localization_node";
        attribute :>> namespaceRef = "/inspection_rover/localization";
        attribute :>> parameterType = float;
        attribute :>> defaultFloat = 20.0;
    }

    part def UseSimTimeParam :> RosParameter {
        attribute :>> name = "use_sim_time";
        attribute :>> ownerNodeRef = "localization_node";
        attribute :>> namespaceRef = "/inspection_rover";
        attribute :>> parameterType = bool;
        attribute :>> defaultBool = false;
    }

    part def GoalToleranceParam :> RosParameter {
        attribute :>> name = "goal_tolerance_m";
        attribute :>> ownerNodeRef = "navigation_node";
        attribute :>> namespaceRef = "/inspection_rover/navigation";
        attribute :>> parameterType = float;
        attribute :>> defaultFloat = 0.25;
    }

    part def MissionGoalFrameParam :> RosParameter {
        attribute :>> name = "goal_frame";
        attribute :>> ownerNodeRef = "mission_supervisor_node";
        attribute :>> namespaceRef = "/inspection_rover/mission";
        attribute :>> parameterType = text;
        attribute :>> defaultText = "map";
    }

    part def OdomFrame :> RosTransformFrame {
        attribute :>> name = "odom";
        attribute :>> parentFrameRef = "map";
    }

    part def BaseLinkFrame :> RosTransformFrame {
        attribute :>> name = "base_link";
        attribute :>> parentFrameRef = "odom";
    }

    part def RoverLifecycle :> LifecycleState {
        attribute :>> phase = active;
    }

    part def RoverPackage :> RosPackage {
        attribute :>> name = "inspection_rover_bringup";
    }

    part def RoverLaunch :> RosLaunchDescription {
        attribute :>> name = "inspection_rover.launch.py";
        attribute :>> launchGroupName = "core_runtime";
        attribute :>> packageRef = "inspection_rover_bringup";
    }

    part def RoverDeploymentUnit :> RosDeploymentUnit {
        attribute :>> packageRef = "inspection_rover_bringup";
    }

    part def LidarDeployment :> RoverDeploymentUnit {
        attribute :>> name = "lidar_deployment";
        attribute :>> executableName = "lidar_node";
        attribute :>> nodeRef = "lidar_node";
    }

    part def LocalizationDeployment :> RoverDeploymentUnit {
        attribute :>> name = "localization_deployment";
        attribute :>> executableName = "localization_node";
        attribute :>> nodeRef = "localization_node";
    }

    part def MotionControllerDeployment :> RoverDeploymentUnit {
        attribute :>> name = "motion_controller_deployment";
        attribute :>> executableName = "motion_controller_node";
        attribute :>> nodeRef = "motion_controller_node";
    }

    part def NavigationDeployment :> RoverDeploymentUnit {
        attribute :>> name = "navigation_deployment";
        attribute :>> executableName = "navigation_node";
        attribute :>> nodeRef = "navigation_node";
    }

    part def MissionSupervisorDeployment :> RoverDeploymentUnit {
        attribute :>> name = "mission_supervisor_deployment";
        attribute :>> executableName = "mission_supervisor_node";
        attribute :>> nodeRef = "mission_supervisor_node";
    }

    part def OperatorConsoleDeployment :> RoverDeploymentUnit {
        attribute :>> name = "operator_console_deployment";
        attribute :>> executableName = "operator_console_node";
        attribute :>> nodeRef = "operator_console_node";
    }

    part def RoverDeployment {
        part roverPackage : RoverPackage;
        part roverLaunch : RoverLaunch;
        part operatorDeploy : OperatorConsoleDeployment;
        part missionDeploy : MissionSupervisorDeployment;
        part lidarDeploy : LidarDeployment;
        part localizationDeploy : LocalizationDeployment;
        part navigationDeploy : NavigationDeployment;
        part motionDeploy : MotionControllerDeployment;
    }

    part def ScanArtifact :> RosGenerationArtifact {
        attribute :>> name = "scan_message";
        attribute :>> artifactKind = interface_msg;
        attribute :>> artifactPath = "interfaces/msg/LaserScan.msg";
    }

    part def RuntimePackageManifest :> RosGenerationArtifact {
        attribute :>> name = "runtime_package_manifest";
        attribute :>> artifactKind = package_manifest;
        attribute :>> artifactPath = "inspection_rover_bringup/package.xml";
    }

    part def RuntimeLaunchArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_launch";
        attribute :>> artifactKind = launch_file;
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover.launch.py";
    }

    part def RuntimeParamsArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_params";
        attribute :>> artifactKind = parameter_file;
        attribute :>> artifactPath = "inspection_rover_bringup/config/runtime.yaml";
    }

    part def TraceScanTopic :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.topic.scan";
        attribute :>> artifactPath = "interfaces/msg/LaserScan.msg";
    }

    part def TraceRuntimeLaunch :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.launch.runtime";
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover.launch.py";
    }

    part def RoverTraceability {
        part scanArtifact : ScanArtifact;
        part packageManifestArtifact : RuntimePackageManifest;
        part launchArtifact : RuntimeLaunchArtifact;
        part paramsArtifact : RuntimeParamsArtifact;
        part traceScan : TraceScanTopic;
        part traceLaunch : TraceRuntimeLaunch;
    }
}
