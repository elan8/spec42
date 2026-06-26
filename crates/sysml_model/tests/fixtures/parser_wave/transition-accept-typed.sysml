package TransitionAcceptDemo {
	state def TimerStates {
		then idle;
		transition to_running first idle accept StartPressed then running;
		transition typed_accept first idle accept evt : StartEvent then running;
	}
}
