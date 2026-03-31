package StateMachineDemo {
    action def StartPressed;
    action def StopPressed;
    action def ResetPressed;
    action def CountdownComplete;

    state def Idle;
    state def Running;
    state def Paused;
    state def Expired;

    state def TimerStateMachine {
        state idle : Idle;
        state running : Running;
        state paused : Paused;
        state expired : Expired;

        transition to_running first idle accept StartPressed then running;
        transition to_paused first running accept StopPressed then paused;
        transition to_expired first running accept CountdownComplete then expired;
        transition to_running_resume first paused accept StartPressed then running;
        transition to_idle_from_paused first paused accept ResetPressed then idle;
        transition to_idle_from_expired first expired accept ResetPressed then idle;
    }
}
