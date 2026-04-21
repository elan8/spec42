package DistributedSystems {
    import SoftwareCore::*;

    part def Service specializes DeployableComponent {
        attribute serviceName : Name;
    }

    part def ServiceInstance specializes Service;
    part def Workload specializes DeployableComponent {
        attribute workloadName : Name;
    }
    part def DomainBoundary {
        attribute boundaryName : Name;
    }
    part def ServiceDependency {
        attribute dependencyName : Name;
    }

    item def Event specializes MessageType;
    item def Command specializes MessageType;
    item def Query specializes MessageType;

    part def Topic {
        attribute topicName : Name;
    }

    part def Subscription;
    part def StreamChannel {
        attribute channelName : Name;
    }

    part def MessageBroker specializes DeployableComponent;

    part def DataStore specializes DeployableComponent {
        attribute storeName : Name;
    }
}
