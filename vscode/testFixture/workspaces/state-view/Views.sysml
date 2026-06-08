package Views {
    import StateMachineDemo::*;

    view timerStateMachine : StateTransitionView {
        doc /* Timer modes: Idle, Running, Paused, Expired and transitions. */
        expose StateMachineDemo::TimerStateMachine;
    }
}
