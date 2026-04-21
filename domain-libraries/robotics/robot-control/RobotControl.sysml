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

    part def TrajectoryController specializes Controller;

    part def MotionCommand specializes ActuationCommand;

    part def Setpoint {
        attribute setpointName : Name;
    }

    part def FeedbackSignal {
        attribute signalName : Name;
        attribute feedbackKind : FeedbackKind;
    }
}
