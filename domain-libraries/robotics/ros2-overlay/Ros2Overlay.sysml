package Ros2Overlay {
    import RoboticsCore::*;
    import RobotRuntime::*;

    attribute def TopicName;
    attribute def ServiceName;
    attribute def ActionName;
    attribute def ParameterName;
    attribute def PackageName;
    attribute def FrameName;
    attribute def MessageTypeRef;
    attribute def NodeRef;
    attribute def TopicRef;
    attribute def QosProfileRef;
    attribute def ParentFrameRef;
    attribute def ProfileName;
    attribute def ReliabilityKind;
    attribute def DurabilityKind;
    attribute def HistoryDepth;

    part def RosNode specializes RuntimeNode;

    part def RosTopic {
        attribute topicName : TopicName;
        attribute messageTypeRef : MessageTypeRef;
    }

    part def RosPublisher {
        attribute publisherName : Name;
        attribute publisherNode : NodeRef;
        attribute topicRef : TopicRef;
        attribute messageTypeRef : MessageTypeRef;
        attribute qosProfileRef : QosProfileRef;
    }

    part def RosSubscriber {
        attribute subscriberName : Name;
        attribute subscriberNode : NodeRef;
        attribute topicRef : TopicRef;
        attribute messageTypeRef : MessageTypeRef;
        attribute qosProfileRef : QosProfileRef;
    }

    part def RosService {
        attribute serviceName : ServiceName;
    }

    part def RosClient {
        attribute clientName : Name;
    }

    part def RosActionServer {
        attribute actionName : ActionName;
    }

    part def RosActionClient {
        attribute actionName : ActionName;
    }

    part def RosParameter {
        attribute parameterName : ParameterName;
    }

    part def RosLifecycleNode specializes RosNode;

    item def RosMessageType {
        attribute messageName : Name;
    }

    part def RosPackage {
        attribute packageName : PackageName;
    }

    part def RosLaunchDescription {
        attribute launchName : Name;
    }

    part def RosTransformFrame {
        attribute frameName : FrameName;
        attribute parentFrameRef : ParentFrameRef;
    }

    part def RosQosProfile {
        attribute profileName : ProfileName;
        attribute reliability : ReliabilityKind;
        attribute durability : DurabilityKind;
        attribute depth : HistoryDepth;
    }
}
