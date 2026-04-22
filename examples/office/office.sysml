/*
 * Office workstation - learner-focused SysML v2 worked example.
 * Focus: small structure, typed interfaces, simple behavior,
 * and basic requirements traceability.
 */
package Office {
	doc /* Root package for a compact office workstation teaching example. */

	// ========== Standard library imports ==========
	private import ScalarValues::Real;
	private import ScalarValues::String;
	private import ScalarValues::Boolean;

	// ========== Port definitions ==========
	port def ACOutletPort {
		out powerAvailable : Boolean;
	}

	port def HdmiPort {
		out videoSignal : String;
	}

	port def UsbHostPort {
		out usbData : String;
	}

	// ========== Part definitions ==========
	part def WallSocket {
		doc /* Building mains outlet powering office equipment. */
		port power : ACOutletPort;
	}

	part def Laptop {
		doc /* Main compute platform for engineering work. */
		attribute ramGb : Real;
		attribute storageGb : Real;
		attribute hasSsd : Boolean;
		port hdmi : HdmiPort;
		port power : ~ACOutletPort;
		port usb1 : UsbHostPort;
		port usb2 : UsbHostPort;
		exhibit state sessionState : WorkSessionStateMachine;
	}

	part def Mouse {
		port usb : ~UsbHostPort;
	}

	part def Keyboard {
		port usb : ~UsbHostPort;
	}

	part def Monitor {
		attribute diagonalInches : Real;
		port hdmi : ~HdmiPort;
		port power : ~ACOutletPort;
	}

	part def OfficeDeskSetup {
		doc /* Laptop plus peripherals and wall power. */
		part wallSocket : WallSocket;
		part laptop : Laptop;
		part mouse : Mouse;
		part keyboard : Keyboard;
		part monitor : Monitor;

		connect laptop.hdmi to monitor.hdmi;
		connect laptop.power to wallSocket.power;
		connect laptop.usb1 to mouse.usb;
		connect laptop.usb2 to keyboard.usb;
		connect monitor.power to wallSocket.power;
	}

	view structure : GeneralView{
		expose OfficeDeskSetup;
	}

	view connections : InterconnectionView{
		expose OfficeDeskSetup;
	}

	// ========== Behavior ==========
	item def StartWork;
	item def LockScreen;
	item def ResumeWork;
	item def ShutdownWorkstation;

	state def ActiveSession {
		doc /* User is actively working. */
	}

	state def IdleLocked {
		doc /* Session locked while workstation remains powered. */
	}

	state def PoweredOff {
		doc /* Workstation is shut down. */
	}

	state def WorkSessionStateMachine {
		entry;
		then poweredOff;
		state active : ActiveSession;
		state idleLocked : IdleLocked;
		state poweredOff : PoweredOff;
		transition boot first poweredOff accept StartWork then active;
		transition lock first active accept LockScreen then idleLocked;
		transition unlock first idleLocked accept ResumeWork then active;
		transition shutdown_from_active first active accept ShutdownWorkstation then poweredOff;
		transition shutdown_from_idle first idleLocked accept ShutdownWorkstation then poweredOff;
	}

	// ========== Requirements ==========
	requirement def ProductivityComputerReq {
		subject laptop : Laptop;
		doc /* Laptop shall provide sufficient performance for engineering analysis workloads. */
		require constraint { doc /* hasSsd = true and ramGb >= 16 and storageGb >= 512. */ }
	}

	requirement def ExternalDisplayReq {
		subject setup : OfficeDeskSetup;
		doc /* Workstation shall support an external display connection. */
		require constraint { doc /* laptop.hdmi connected to monitor.hdmi. */ }
	}

	requirement def InputDeviceReq {
		subject setup : OfficeDeskSetup;
		doc /* Workstation shall include keyboard and pointing device connectivity. */
		require constraint { doc /* both USB host ports connected to peripherals. */ }
	}

	requirement def ContinuousPowerReq {
		subject setup : OfficeDeskSetup;
		doc /* Laptop and monitor shall receive mains power from wall socket. */
		require constraint { doc /* laptop.power and monitor.power connected to wallSocket.power. */ }
	}

	part officeInstance : OfficeDeskSetup;
	satisfy ProductivityComputerReq by officeInstance.laptop;
	satisfy ExternalDisplayReq by officeInstance;
	satisfy InputDeviceReq by officeInstance;
	satisfy ContinuousPowerReq by officeInstance;

	// ========== Use cases ==========
	item def OfficeUser;

	use case def StartDeskWork {
		subject setup : OfficeDeskSetup;
		actor user : OfficeUser;
		doc /* User powers on laptop and starts the work session. */
		objective { doc /* Reach ActiveSession with all required peripherals connected. */ }
	}

	use case def LockAndResume {
		subject setup : OfficeDeskSetup;
		actor user : OfficeUser;
		doc /* User locks workstation while away, then resumes later. */
		objective { doc /* Protect session while maintaining desk setup readiness. */ }
	}
}
