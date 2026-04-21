package MissingDeploymentExample {
    import SoftwareCore::*;
    import DistributedSystems::*;

    interface def BillingApi :> ApiInterface;

    part def BillingService :> Service {
        attribute serviceName = "billing";
    }

    part def CustomerService :> Service {
        attribute serviceName = "customer";
    }

    part def RuntimeNode :> DeploymentNode {
        attribute nodeName = "runtime-a";
        attribute host = "runtime-a.internal";
    }

    allocate CustomerService to RuntimeNode;
}
