package RobotControl {
    import RoboticsCore::*;
    import RobotActuation::*;

    attribute def ControlType;
    attribute def FeedbackKind;

    part def ControlLoop {
        attribute loopName : Name;
        attribute controlType : ControlType;
    }

    part def Controller {
        attribute controllerName : Name;
    }

    part def TrajectoryController :> Controller;

    part def MotionCommand :> ActuationCommand;

    part def Setpoint {
        attribute setpointName : Name;
    }

    part def FeedbackSignal {
        attribute signalName : Name;
        attribute feedbackKind : FeedbackKind;
    }
}
