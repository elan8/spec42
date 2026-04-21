package InspectionRoverExample {
    import RoboticsCore::*;
    import RobotRuntime::*;
    import RobotStructure::*;
    import Ros2Overlay::*;

    // Runtime responsibility map:
    // - operator_console_node: publishes mission GPS targets from user/operator intent.
    // - mission_supervisor_node: translates GPS mission targets into map-frame navigation goals
    //   and executes inspection workflow once goal arrival is confirmed.
    // - localization_node: robot_localization EKF estimates current robot pose and publishes odometry.
    // - navigation_node: Nav2 BT navigator orchestrates planner/controller/behavior servers.
    // - planner_node/controller_node/behavior_node: Nav2 servers for path planning and execution.
    // - lifecycle_manager_node: activates/deactivates managed Nav2 lifecycle nodes.
    // - motion_controller_node: ros2_control controller manager converts velocity commands to motor actuation.

    part def TurtleBot4Platform :> RobotPlatform {
        attribute :>> name = "turtlebot4_base";
        attribute :>> criticality = medium;
    }

    part def InspectionRover :> TurtleBot4Platform {
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
        part plannerRuntime : Nav2PlannerNode;
        part controllerRuntime : Nav2ControllerNode;
        part behaviorRuntime : Nav2BehaviorNode;
        part lifecycleRuntime : Nav2LifecycleManagerNode;
        part motionRuntime : MotionControllerNode;

        connect operatorRuntime.gpsTargetPubPort to missionRuntime.gpsTargetSubPort;
        connect missionRuntime.goalPubPort to navigationRuntime.goalSubPort;
        connect navigationRuntime.planRequestPubPort to plannerRuntime.planRequestSubPort;
        connect plannerRuntime.pathPubPort to controllerRuntime.pathSubPort;
        connect localizationRuntime.odomPubPort to controllerRuntime.odomSubPort;
        connect lidarRuntime.scanPubPort to localizationRuntime.scanSubPort;
        connect localizationRuntime.odomPubPort to navigationRuntime.odomSubPort;
        connect controllerRuntime.cmdVelPubPort to motionRuntime.cmdVelSubPort;
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
        attribute :>> packageRef = "robot_localization";
        attribute :>> executableName = "ekf_node";

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
        attribute :>> packageRef = "nav2_bt_navigator";
        attribute :>> executableName = "bt_navigator";

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

        port planRequestPubPort : RosPublishPort {
            attribute :>> name = "plan_request_pub";
            attribute :>> ownerNodeRef = "navigation_node";
            attribute :>> topicRef = "/plan_request";
            attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
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

    part def Nav2PlannerNode :> RoverRuntimeNode {
        attribute :>> name = "planner_node";
        attribute :>> packageRef = "nav2_planner";
        attribute :>> executableName = "planner_server";

        port planRequestSubPort : RosSubscribePort {
            attribute :>> name = "plan_request_sub";
            attribute :>> ownerNodeRef = "planner_node";
            attribute :>> topicRef = "/plan_request";
            attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port pathPubPort : RosPublishPort {
            attribute :>> name = "path_pub";
            attribute :>> ownerNodeRef = "planner_node";
            attribute :>> topicRef = "/planned_path";
            attribute :>> messageTypeRef = "nav_msgs/Path";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    part def Nav2ControllerNode :> RoverRuntimeNode {
        attribute :>> name = "controller_node";
        attribute :>> packageRef = "nav2_controller";
        attribute :>> executableName = "controller_server";

        port pathSubPort : RosSubscribePort {
            attribute :>> name = "path_sub";
            attribute :>> ownerNodeRef = "controller_node";
            attribute :>> topicRef = "/planned_path";
            attribute :>> messageTypeRef = "nav_msgs/Path";
            attribute :>> qosProfileRef = "control_reliable";
        }

        port odomSubPort : RosSubscribePort {
            attribute :>> name = "odom_sub";
            attribute :>> ownerNodeRef = "controller_node";
            attribute :>> topicRef = "/odom";
            attribute :>> messageTypeRef = "nav_msgs/Odometry";
            attribute :>> qosProfileRef = "sensor_data";
        }

        port cmdVelPubPort : RosPublishPort {
            attribute :>> name = "cmd_vel_pub";
            attribute :>> ownerNodeRef = "controller_node";
            attribute :>> topicRef = "/cmd_vel";
            attribute :>> messageTypeRef = "geometry_msgs/Twist";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    part def Nav2BehaviorNode :> RoverRuntimeNode {
        attribute :>> name = "behavior_node";
        attribute :>> packageRef = "nav2_behaviors";
        attribute :>> executableName = "behavior_server";
    }

    part def Nav2LifecycleManagerNode :> RoverRuntimeNode {
        attribute :>> name = "lifecycle_manager_node";
        attribute :>> packageRef = "nav2_lifecycle_manager";
        attribute :>> executableName = "lifecycle_manager";
    }

    part def MotionControllerNode :> RoverCoreRuntimeNode {
        attribute :>> name = "motion_controller_node";
        attribute :>> packageRef = "controller_manager";
        attribute :>> executableName = "ros2_control_node";

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

    item def PathMessage :> RosMessageType {
        attribute :>> name = "nav_msgs/Path";
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

    part def PlanRequestTopic :> RosTopic {
        attribute :>> name = "/plan_request";
        attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
    }

    part def PlannedPathTopic :> RosTopic {
        attribute :>> name = "/planned_path";
        attribute :>> messageTypeRef = "nav_msgs/Path";
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

    part def TfTopic :> RosTopic {
        attribute :>> name = "/tf";
        attribute :>> messageTypeRef = "tf2_msgs/TFMessage";
    }

    part def SensorQos :> RosQosProfile {
        attribute :>> name = "sensor_data";
        attribute :>> reliability = best_effort;
    }

    part def ControlQos :> RosQosProfile {
        attribute :>> name = "control_reliable";
        attribute :>> reliability = reliable;
    }

    part def NavigationRemapScan :> RosRemapRule {
        attribute :>> name = "navigation_scan_remap";
        attribute :>> sourceName = "/scan";
        attribute :>> targetName = "/inspection_rover/scan";
        attribute :>> ownerNodeRef = "navigation_node";
    }

    part def MissionRemapGoal :> RosRemapRule {
        attribute :>> name = "mission_goal_remap";
        attribute :>> sourceName = "/goal_pose";
        attribute :>> targetName = "/inspection_rover/goal_pose";
        attribute :>> ownerNodeRef = "mission_supervisor_node";
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
        attribute :>> publisherNode = "controller_node";
        attribute :>> topicRef = "/cmd_vel";
        attribute :>> messageTypeRef = "geometry_msgs/Twist";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def PlanRequestPublisher :> RosPublisher {
        attribute :>> name = "navigation_plan_request_pub";
        attribute :>> publisherNode = "navigation_node";
        attribute :>> topicRef = "/plan_request";
        attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def PlanRequestSubscriber :> RosSubscriber {
        attribute :>> name = "planner_plan_request_sub";
        attribute :>> subscriberNode = "planner_node";
        attribute :>> topicRef = "/plan_request";
        attribute :>> messageTypeRef = "geometry_msgs/PoseStamped";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def PlannedPathPublisher :> RosPublisher {
        attribute :>> name = "planner_path_pub";
        attribute :>> publisherNode = "planner_node";
        attribute :>> topicRef = "/planned_path";
        attribute :>> messageTypeRef = "nav_msgs/Path";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def PlannedPathSubscriber :> RosSubscriber {
        attribute :>> name = "controller_path_sub";
        attribute :>> subscriberNode = "controller_node";
        attribute :>> topicRef = "/planned_path";
        attribute :>> messageTypeRef = "nav_msgs/Path";
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

    part def Nav2ManagedNodesParam :> RosParameter {
        attribute :>> name = "managed_nodes";
        attribute :>> ownerNodeRef = "lifecycle_manager_node";
        attribute :>> namespaceRef = "/inspection_rover/nav2/lifecycle_manager_navigation";
        attribute :>> parameterType = text;
        attribute :>> defaultText = "[bt_navigator,planner_server,controller_server,behavior_server]";
    }

    part def Nav2BtXmlParam :> RosParameter {
        attribute :>> name = "default_bt_xml_filename";
        attribute :>> ownerNodeRef = "navigation_node";
        attribute :>> namespaceRef = "/inspection_rover/nav2/bt_navigator";
        attribute :>> parameterType = text;
        attribute :>> defaultText = "config/navigate_to_pose_w_replanning_and_recovery.xml";
    }

    part def Nav2PlannerPluginParam :> RosParameter {
        attribute :>> name = "planner_plugins";
        attribute :>> ownerNodeRef = "planner_node";
        attribute :>> namespaceRef = "/inspection_rover/nav2/planner_server";
        attribute :>> parameterType = text;
        attribute :>> defaultText = "[GridBased]";
    }

    part def Nav2ControllerPluginParam :> RosParameter {
        attribute :>> name = "controller_plugins";
        attribute :>> ownerNodeRef = "controller_node";
        attribute :>> namespaceRef = "/inspection_rover/nav2/controller_server";
        attribute :>> parameterType = text;
        attribute :>> defaultText = "[FollowPath]";
    }

    part def Nav2GlobalCostmapResolutionParam :> RosParameter {
        attribute :>> name = "global_costmap_resolution";
        attribute :>> ownerNodeRef = "planner_node";
        attribute :>> namespaceRef = "/inspection_rover/nav2/global_costmap";
        attribute :>> parameterType = float;
        attribute :>> defaultFloat = 0.05;
    }

    part def Nav2LocalCostmapResolutionParam :> RosParameter {
        attribute :>> name = "local_costmap_resolution";
        attribute :>> ownerNodeRef = "controller_node";
        attribute :>> namespaceRef = "/inspection_rover/nav2/local_costmap";
        attribute :>> parameterType = float;
        attribute :>> defaultFloat = 0.05;
    }

    part def Nav2ConfigurationProfile {
        part lifecycleManager : Nav2ManagedNodesParam;
        part btNavigatorXml : Nav2BtXmlParam;
        part plannerPlugins : Nav2PlannerPluginParam;
        part controllerPlugins : Nav2ControllerPluginParam;
        part globalCostmapResolution : Nav2GlobalCostmapResolutionParam;
        part localCostmapResolution : Nav2LocalCostmapResolutionParam;
    }

    part def RuntimeParameterProfile :> RosParameterProfile {
        attribute :>> name = "runtime_profile";
        attribute :>> parameterFileRef = "config/runtime.yaml";
        attribute :>> ownerNodeRef = "mission_supervisor_node";
    }

    part def Nav2ParameterProfile :> RosParameterProfile {
        attribute :>> name = "nav2_profile";
        attribute :>> parameterFileRef = "config/nav2.yaml";
        attribute :>> ownerNodeRef = "navigation_node";
    }

    part def SecurityParameterProfile :> RosParameterProfile {
        attribute :>> name = "security_profile";
        attribute :>> parameterFileRef = "config/security.yaml";
        attribute :>> ownerNodeRef = "mission_supervisor_node";
    }

    part def Nav2LifecycleConfig :> RosLifecycleManagerConfig {
        attribute :>> name = "nav2_lifecycle_manager";
        attribute :>> managerNodeRef = "lifecycle_manager_node";
        attribute :>> managedNodeOrder = "bt_navigator,planner_server,controller_server,behavior_server";
        attribute :>> targetState = active;
    }

    part def OdomFrame :> RosTransformFrame {
        attribute :>> name = "odom";
        attribute :>> parentFrameRef = "map";
    }

    part def BaseLinkFrame :> RosTransformFrame {
        attribute :>> name = "base_link";
        attribute :>> parentFrameRef = "odom";
    }

    part def MapToOdomProducer :> RosTfProducer {
        attribute :>> name = "map_to_odom_tf";
        attribute :>> sourceFrameRef = "map";
        attribute :>> targetFrameRef = "odom";
        attribute :>> authorityKind = dynamic;
        attribute :>> ownerNodeRef = "localization_node";
    }

    part def OdomToBaseProducer :> RosTfProducer {
        attribute :>> name = "odom_to_base_tf";
        attribute :>> sourceFrameRef = "odom";
        attribute :>> targetFrameRef = "base_link";
        attribute :>> authorityKind = dynamic;
        attribute :>> ownerNodeRef = "localization_node";
    }

    part def RoverLifecycle :> LifecycleState {
        attribute :>> phase = active;
    }

    part def RoverPackage :> RosPackage {
        attribute :>> name = "inspection_rover_bringup";
    }

    part def Nav2Package :> RosPackage {
        attribute :>> name = "nav2_bt_navigator";
    }

    part def TurtleBot4BringupPackage :> RosPackage {
        attribute :>> name = "turtlebot4_bringup";
    }

    part def TurtleBot4IgnitionBringupPackage :> RosPackage {
        attribute :>> name = "turtlebot4_ignition_bringup";
    }

    part def TurtleBot4DescriptionPackage :> RosPackage {
        attribute :>> name = "turtlebot4_description";
    }

    part def Nav2PlannerPackage :> RosPackage {
        attribute :>> name = "nav2_planner";
    }

    part def Nav2ControllerPackage :> RosPackage {
        attribute :>> name = "nav2_controller";
    }

    part def Nav2BehaviorPackage :> RosPackage {
        attribute :>> name = "nav2_behaviors";
    }

    part def Nav2LifecyclePackage :> RosPackage {
        attribute :>> name = "nav2_lifecycle_manager";
    }

    part def RobotLocalizationPackage :> RosPackage {
        attribute :>> name = "robot_localization";
    }

    part def Ros2ControlPackage :> RosPackage {
        attribute :>> name = "controller_manager";
    }

    part def RoverLaunch :> RosLaunchDescription {
        attribute :>> name = "inspection_rover.launch.py";
        attribute :>> launchGroupName = "core_runtime";
        attribute :>> packageRef = "inspection_rover_bringup";
        attribute :>> launchMode = standalone;
    }

    part def RoverComposedLaunch :> RosLaunchDescription {
        attribute :>> name = "inspection_rover_composed.launch.py";
        attribute :>> launchGroupName = "core_runtime";
        attribute :>> packageRef = "inspection_rover_bringup";
        attribute :>> launchMode = composed;
    }

    part def Nav2LaunchInclude :> RosLaunchInclude {
        attribute :>> name = "nav2_bringup_include";
        attribute :>> includeRef = "turtlebot4_bringup/nav2.launch.py";
        attribute :>> conditionExpr = "true";
    }

    part def TurtleBot4RobotLaunchInclude :> RosLaunchInclude {
        attribute :>> name = "turtlebot4_robot_include";
        attribute :>> includeRef = "turtlebot4_bringup/robot.launch.py";
        attribute :>> conditionExpr = "not use_simulation";
    }

    part def TurtleBot4IgnitionLaunchInclude :> RosLaunchInclude {
        attribute :>> name = "turtlebot4_ignition_include";
        attribute :>> includeRef = "turtlebot4_ignition_bringup/ignition.launch.py";
        attribute :>> conditionExpr = "use_simulation";
    }

    part def SecurityLaunchInclude :> RosLaunchInclude {
        attribute :>> name = "security_launch_include";
        attribute :>> includeRef = "inspection_rover_bringup/security_launch.py";
        attribute :>> conditionExpr = "enable_security";
    }

    part def RuntimeLaunchGroup :> RosLaunchGroup {
        attribute :>> name = "runtime_group";
        attribute :>> conditionExpr = "true";
    }

    part def SimulationLaunchGroup :> RosLaunchGroup {
        attribute :>> name = "simulation_group";
        attribute :>> conditionExpr = "use_simulation";
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
        attribute :>> packageRef = "robot_localization";
        attribute :>> executableName = "ekf_node";
        attribute :>> nodeRef = "localization_node";
    }

    part def MotionControllerDeployment :> RoverDeploymentUnit {
        attribute :>> name = "motion_controller_deployment";
        attribute :>> packageRef = "controller_manager";
        attribute :>> executableName = "ros2_control_node";
        attribute :>> nodeRef = "motion_controller_node";
    }

    part def NavigationDeployment :> RoverDeploymentUnit {
        attribute :>> name = "navigation_deployment";
        attribute :>> packageRef = "nav2_bt_navigator";
        attribute :>> executableName = "bt_navigator";
        attribute :>> nodeRef = "navigation_node";
    }

    part def PlannerDeployment :> RoverDeploymentUnit {
        attribute :>> name = "planner_deployment";
        attribute :>> packageRef = "nav2_planner";
        attribute :>> executableName = "planner_server";
        attribute :>> nodeRef = "planner_node";
    }

    part def ControllerDeployment :> RoverDeploymentUnit {
        attribute :>> name = "controller_deployment";
        attribute :>> packageRef = "nav2_controller";
        attribute :>> executableName = "controller_server";
        attribute :>> nodeRef = "controller_node";
    }

    part def BehaviorDeployment :> RoverDeploymentUnit {
        attribute :>> name = "behavior_deployment";
        attribute :>> packageRef = "nav2_behaviors";
        attribute :>> executableName = "behavior_server";
        attribute :>> nodeRef = "behavior_node";
    }

    part def LifecycleManagerDeployment :> RoverDeploymentUnit {
        attribute :>> name = "lifecycle_manager_deployment";
        attribute :>> packageRef = "nav2_lifecycle_manager";
        attribute :>> executableName = "lifecycle_manager";
        attribute :>> nodeRef = "lifecycle_manager_node";
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

    part def MissionSecurityProfile :> RosSecurityProfile {
        attribute :>> name = "mission_security";
        attribute :>> ownerNodeRef = "mission_supervisor_node";
        attribute :>> enclaveRef = "/inspection_rover/mission";
        attribute :>> securityPolicyRef = "security/mission.policy.xml";
        attribute :>> keystorePathRef = "security/keystore";
    }

    part def NavigationSecurityProfile :> RosSecurityProfile {
        attribute :>> name = "navigation_security";
        attribute :>> ownerNodeRef = "navigation_node";
        attribute :>> enclaveRef = "/inspection_rover/navigation";
        attribute :>> securityPolicyRef = "security/navigation.policy.xml";
        attribute :>> keystorePathRef = "security/keystore";
    }

    part def RuntimeExecDependencyNav2 :> RosManifestDependency {
        attribute :>> name = "runtime_dep_nav2";
        attribute :>> ownerPackageRef = "inspection_rover_bringup";
        attribute :>> dependencyKind = exec;
        attribute :>> dependencyPackageRef = "nav2_bt_navigator";
    }

    part def RuntimeExecDependencyLocalization :> RosManifestDependency {
        attribute :>> name = "runtime_dep_localization";
        attribute :>> ownerPackageRef = "inspection_rover_bringup";
        attribute :>> dependencyKind = exec;
        attribute :>> dependencyPackageRef = "robot_localization";
    }

    part def RuntimeExecDependencyControl :> RosManifestDependency {
        attribute :>> name = "runtime_dep_control";
        attribute :>> ownerPackageRef = "inspection_rover_bringup";
        attribute :>> dependencyKind = exec;
        attribute :>> dependencyPackageRef = "controller_manager";
    }

    part def RuntimeExecDependencyTurtleBot4Bringup :> RosManifestDependency {
        attribute :>> name = "runtime_dep_turtlebot4_bringup";
        attribute :>> ownerPackageRef = "inspection_rover_bringup";
        attribute :>> dependencyKind = exec;
        attribute :>> dependencyPackageRef = "turtlebot4_bringup";
    }

    part def RuntimeExecDependencyTurtleBot4Ignition :> RosManifestDependency {
        attribute :>> name = "runtime_dep_turtlebot4_ignition";
        attribute :>> ownerPackageRef = "inspection_rover_bringup";
        attribute :>> dependencyKind = exec;
        attribute :>> dependencyPackageRef = "turtlebot4_ignition_bringup";
    }

    part def RoverDeployment {
        part roverPackage : RoverPackage;
        part nav2Package : Nav2Package;
        part turtlebot4BringupPackage : TurtleBot4BringupPackage;
        part turtlebot4IgnitionPackage : TurtleBot4IgnitionBringupPackage;
        part turtlebot4DescriptionPackage : TurtleBot4DescriptionPackage;
        part nav2PlannerPackage : Nav2PlannerPackage;
        part nav2ControllerPackage : Nav2ControllerPackage;
        part nav2BehaviorPackage : Nav2BehaviorPackage;
        part nav2LifecyclePackage : Nav2LifecyclePackage;
        part localizationPackage : RobotLocalizationPackage;
        part controlPackage : Ros2ControlPackage;
        part roverLaunch : RoverLaunch;
        part roverComposedLaunch : RoverComposedLaunch;
        part nav2Include : Nav2LaunchInclude;
        part turtlebot4RobotInclude : TurtleBot4RobotLaunchInclude;
        part turtlebot4IgnitionInclude : TurtleBot4IgnitionLaunchInclude;
        part securityInclude : SecurityLaunchInclude;
        part runtimeLaunchGroup : RuntimeLaunchGroup;
        part simulationLaunchGroup : SimulationLaunchGroup;
        part nav2ConfigProfile : Nav2ConfigurationProfile;
        part runtimeParamProfile : RuntimeParameterProfile;
        part nav2ParamProfile : Nav2ParameterProfile;
        part securityParamProfile : SecurityParameterProfile;
        part nav2LifecycleManagerConfig : Nav2LifecycleConfig;
        part operatorDeploy : OperatorConsoleDeployment;
        part missionDeploy : MissionSupervisorDeployment;
        part lidarDeploy : LidarDeployment;
        part localizationDeploy : LocalizationDeployment;
        part navigationDeploy : NavigationDeployment;
        part plannerDeploy : PlannerDeployment;
        part controllerDeploy : ControllerDeployment;
        part behaviorDeploy : BehaviorDeployment;
        part lifecycleDeploy : LifecycleManagerDeployment;
        part motionDeploy : MotionControllerDeployment;
        part mapToOdomTf : MapToOdomProducer;
        part odomToBaseTf : OdomToBaseProducer;
        part missionSecurity : MissionSecurityProfile;
        part navigationSecurity : NavigationSecurityProfile;
        part nav2ExecDep : RuntimeExecDependencyNav2;
        part localizationExecDep : RuntimeExecDependencyLocalization;
        part controlExecDep : RuntimeExecDependencyControl;
        part turtlebot4BringupExecDep : RuntimeExecDependencyTurtleBot4Bringup;
        part turtlebot4IgnitionExecDep : RuntimeExecDependencyTurtleBot4Ignition;
        part remapNavScan : NavigationRemapScan;
        part remapMissionGoal : MissionRemapGoal;
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

    part def RuntimeComposedLaunchArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_composed_launch";
        attribute :>> artifactKind = launch_file;
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover_composed.launch.py";
    }

    part def TurtleBot4GazeboLaunchArtifact :> RosGenerationArtifact {
        attribute :>> name = "turtlebot4_gazebo_launch";
        attribute :>> artifactKind = launch_file;
        attribute :>> artifactPath = "inspection_rover_bringup/launch/turtlebot4_ignition.launch.py";
    }

    part def RuntimeParamsArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_params";
        attribute :>> artifactKind = parameter_file;
        attribute :>> artifactPath = "inspection_rover_bringup/config/runtime.yaml";
    }

    part def Nav2ParamsArtifact :> RosGenerationArtifact {
        attribute :>> name = "nav2_params";
        attribute :>> artifactKind = parameter_file;
        attribute :>> artifactPath = "inspection_rover_bringup/config/nav2.yaml";
    }

    part def SecurityParamsArtifact :> RosGenerationArtifact {
        attribute :>> name = "security_params";
        attribute :>> artifactKind = parameter_file;
        attribute :>> artifactPath = "inspection_rover_bringup/config/security.yaml";
    }

    part def RuntimeCmakeArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_cmake";
        attribute :>> artifactKind = package_manifest;
        attribute :>> artifactPath = "inspection_rover_bringup/CMakeLists.txt";
    }

    part def RuntimeInterfacesArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_interfaces";
        attribute :>> artifactKind = interface_msg;
        attribute :>> artifactPath = "inspection_rover_interfaces/msg/MissionTarget.msg";
    }

    part def SecurityPolicyArtifact :> RosGenerationArtifact {
        attribute :>> name = "security_policy";
        attribute :>> artifactKind = package_manifest;
        attribute :>> artifactPath = "inspection_rover_bringup/security/permissions.xml";
    }

    part def TraceScanTopic :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.topic.scan";
        attribute :>> artifactPath = "interfaces/msg/LaserScan.msg";
    }

    part def TraceRuntimeLaunch :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.launch.runtime";
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover.launch.py";
    }

    part def TraceRuntimeComposedLaunch :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.launch.runtime.composed";
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover_composed.launch.py";
    }

    part def TraceTurtleBot4GazeboLaunch :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.launch.turtlebot4.ignition";
        attribute :>> artifactPath = "inspection_rover_bringup/launch/turtlebot4_ignition.launch.py";
    }

    part def TraceNav2Params :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.config.nav2";
        attribute :>> artifactPath = "inspection_rover_bringup/config/nav2.yaml";
    }

    part def TraceSecurityParams :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.config.security";
        attribute :>> artifactPath = "inspection_rover_bringup/config/security.yaml";
    }

    part def TraceRuntimeCmake :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.manifest.runtime.cmake";
        attribute :>> artifactPath = "inspection_rover_bringup/CMakeLists.txt";
    }

    part def TraceRuntimeInterfaces :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.interfaces.runtime";
        attribute :>> artifactPath = "inspection_rover_interfaces/msg/MissionTarget.msg";
    }

    part def TraceSecurityPolicy :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.security.policy";
        attribute :>> artifactPath = "inspection_rover_bringup/security/permissions.xml";
    }

    part def RoverTraceability {
        part scanArtifact : ScanArtifact;
        part packageManifestArtifact : RuntimePackageManifest;
        part launchArtifact : RuntimeLaunchArtifact;
        part composedLaunchArtifact : RuntimeComposedLaunchArtifact;
        part turtlebot4GazeboLaunchArtifact : TurtleBot4GazeboLaunchArtifact;
        part paramsArtifact : RuntimeParamsArtifact;
        part nav2ParamsArtifact : Nav2ParamsArtifact;
        part securityParamsArtifact : SecurityParamsArtifact;
        part runtimeCmakeArtifact : RuntimeCmakeArtifact;
        part runtimeInterfacesArtifact : RuntimeInterfacesArtifact;
        part securityPolicyArtifact : SecurityPolicyArtifact;
        part traceScan : TraceScanTopic;
        part traceLaunch : TraceRuntimeLaunch;
        part traceComposedLaunch : TraceRuntimeComposedLaunch;
        part traceTurtleBot4Gazebo : TraceTurtleBot4GazeboLaunch;
        part traceNav2Config : TraceNav2Params;
        part traceSecurityConfig : TraceSecurityParams;
        part traceRuntimeCmake : TraceRuntimeCmake;
        part traceInterfaces : TraceRuntimeInterfaces;
        part traceSecurityPolicy : TraceSecurityPolicy;
    }
}
