library package SoftwareInteractions {
    part def InteractionScenario;

    part def Lifeline;

    part def Message {
        ref from : Lifeline;
        ref to : Lifeline;
        attribute label : String;
    }

    part def SynchronousCall :> Message;
    part def AsynchronousMessage :> Message;
    part def ReturnMessage :> Message;
    part def CreationMessage :> Message;

    part def Activation {
        ref on : Lifeline;
        ref startMessage : Message;
        ref finishMessage : Message;
    }

    part def CombinedFragment;
    part def AltFragment :> CombinedFragment;
    part def OptFragment :> CombinedFragment;
    part def LoopFragment :> CombinedFragment;

    part def InteractionOperand {
        attribute guard : String;
    }

    part def InteractionRef :> CombinedFragment {
        ref target : InteractionScenario;
    }
}
