package MissingDeploymentExample {
    import SoftwareCore::*;
    import DistributedSystems::*;

    port def DependencyPort;

    interface def BillingApi :> ApiInterface;

    part def BillingService :> Service {
        attribute :>> serviceName = "billing";
        port billingOut : DependencyPort;
    }

    part def CustomerService :> Service {
        attribute :>> serviceName = "customer";
        port customerIn : DependencyPort;
    }

    part def RuntimeNode :> DeploymentNode {
        attribute :>> nodeName = "runtime-a";
        attribute :>> host = "runtime-a.internal";
    }

    part def MissingDeploymentSystem {
        part billingService : BillingService;
        part customerService : CustomerService;
        part runtimeNode : RuntimeNode;

        connect billingService.billingOut to customerService.customerIn;
    }

    allocate CustomerService to RuntimeNode;
}
