# META
~~~ini
description=SysML Example (Vehicle): SysML v2 Spec Annex A SimpleVehicleModel
type=file
~~~
# SOURCE
~~~sysml
package SimpleVehicleModel{
    // 2023-02 release
    public import Definitions::*;  
    public import ISQ::*;
    package Definitions{
        public import PartDefinitions::*;
        public import PortDefinitions::*;
        public import ItemDefinitions::*;
        public import SignalDefinitions::*;
        public import InterfaceDefinitions::*;
        public import AllocationDefinitions::*;
        public import ActionDefinitions::*;
        public import StateDefinitions::*;
        public import RequirementDefinitions::*;
        public import AttributeDefinitions::*;
        public import IndividualDefinitions::*;
        public import MetadataDefinitions::**;
        public import KeyWord_MetadataDefinitions::*;
        package PartDefinitions{
            part def Vehicle {
                attribute mass :> ISQ::mass;
                attribute dryMass:>ISQ::mass;
                attribute cargoMass:>ISQ::mass;
                attribute position:>ISQ::length;
                attribute velocity:>ISQ::speed;
                attribute acceleration:>ISQ::acceleration;
                attribute electricalPower:>ISQ::power;
                attribute Tmax:>ISQ::temperature;
                attribute maintenanceTime: Time::DateTime; 
                attribute brakePedalDepressed: Boolean;
                port ignitionCmdPort:IgnitionCmdPort;
                port pwrCmdPort:PwrCmdPort;
                port vehicleToRoadPort:VehicleToRoadPort;
                port statusPort:StatusPort;
                perform action providePower;
                perform action provideBraking;
                perform action controlDirection;
                perform action performSelfTest;
                perform action applyParkingBrake;
                perform action senseTemperature;
                exhibit state vehicleStates parallel {
                    ref controller : VehicleController;
                    state operatingStates {
                        entry action initial;
                        state off;                    
                        state starting;                    
                        state on {
                            entry performSelfTest;
                            do providePower;
                            exit applyParkingBrake;
                            constraint {electricalPower<=500[W]}
                        }

                        transition initial then off;

                        transition off_To_starting
                            first off
                            accept ignitionCmd:IgnitionCmd via ignitionCmdPort
                                if ignitionCmd.ignitionOnOff==IgnitionOnOff::on and brakePedalDepressed
                            do send new StartSignal() to controller
                            then starting;
                        
                        transition starting_To_on
                            first starting
                            accept VehicleOnSignal
                            then on;
                        
                        transition on_To_off
                            first on
                            accept VehicleOffSignal
                            do send new OffSignal() to controller
                            then off;
                    }

                    state healthStates {
                        entry action initial;
                        do senseTemperature{
                            out temp;
                        }

                        state normal;
                        state maintenance;
                        state degraded;                    

                        transition initial then normal;

                        transition normal_To_maintenance
                            first normal
                            accept at maintenanceTime
                            then maintenance;

                        transition normal_To_degraded
                            first normal
                            accept when senseTemperature.temp > Tmax 
                            do send new OverTemp() to controller
                            then degraded;

                        transition maintenance_To_normal
                            first maintenance
                            accept ReturnToNormal
                            then normal;

                        transition degraded_To_normal
                            first degraded
                            accept ReturnToNormal
                            then normal;
                    }
                }
            }
            part def Engine{
                attribute mass :> ISQ::mass;
                attribute peakHorsePower:>ISQ::power;
                attribute fuelEfficiency:Real;
                attribute cost:Real;
                attribute displacement :> ISQ::volume;
                port engineControlPort: ~ControlPort;
                port fuelInPort: ~ FuelPort;
                port fuelCmdPort:FuelCmdPort;
                port drivePwrPort:DrivePwrPort;
                port ignitionCmdPort:IgnitionCmdPort;
                port flyWheelPort;
                perform action generateTorque;
                exhibit state engineStates{
                    state off;
                    state starting;
                    state on{
                        do generateTorque;
                    }
                }
            }
            part def StarterMotor{
                port gearPort:GearPort;
            }
            part def Cylinder;
            part def Transmission{
                attribute gearRatio:Real;
                port clutchPort:~DrivePwrPort;
                exhibit state transmissionStates;
            }
            part def Driveshaft;
            part def AxleAssembly;
            part def Axle{
                attribute mass:>ISQ::mass;
            }
            part def FrontAxle:>Axle{
                attribute steeringAngle:>ISQ::angularMeasure;
            }
            part def HalfAxle{
                port shankCompositePort:ShankCompositePort{
                }
            }
            part def Differential;
            part def Wheel{
                attribute diameter:LengthValue;
                port lugNutCompositePort:LugNutCompositePort;
            }
            part def Hub{
                port shankCompositePort:ShankCompositePort;
            }
            abstract part def Software;
            part def VehicleSoftware:>Software;
            part def VehicleController:>Software {
                port controlPort:ControlPort;
                exhibit state controllerStates parallel {
                    state operatingStates {
                        entry action initial; 
                        state off;
                        state on;    
                        transition initial then off;
                        transition 'off-on'
                            first off
                            accept StartSignal
                            then on;
                        transition 'on-off'
                            first on
                            accept OffSignal
                            then off;
                    }
                }  
            }
            part def CruiseController:>Software {
                port setSpeedPort:~SetSpeedPort;
                port speedSensorPort:~SpeedSensorPort;
                port cruiseControlPort:CruiseControlPort;
                exhibit state cruiseControllerStates;
            }
            part def SpeedSensor{
                port speedSensorPort:SpeedSensorPort;
            }
            part def FuelTank{
                attribute mass :> ISQ::mass;
                ref item fuel:Fuel{
                    attribute :>> fuelMass;
                }
                attribute fuelKind:FuelKind;
                attribute fuelMassMax:>ISQ::mass;
                assert constraint fuelConstraint {fuel.fuelMass<=fuelMassMax}
                port fuelOutPort:FuelPort;
                port fuelInPort:~FuelPort;
            }
            part def BodyAssy;
            part def Body{
                attribute color:Colors;
            }
            part def Thermostat;
            part def WaterHose;
            part def Road{
                attribute incline:Real;
                attribute friction:Real;
            }
            part def Engine4Cyl;
            part def Engine6Cyl;
            part def TransmissionChoices;
            part def TransmissionAutomatic;
            part def TransmissionManual;
            part def Sunroof;
            
            //logical Components
            part def ElectricalGenerator;
            part def TorqueGenerator;
            part def SteeringSubsystem;
            part def BrakingSubsystem;
        }
        package PortDefinitions{
            port def IgnitionCmdPort{
                in item ignitionCmd:IgnitionCmd;
            }
            port def StatusPort;
            port def GearPort;
            port def PwrCmdPort{
                in item pwrCmd:PwrCmd;
            }
            port def FuelCmdPort:>PwrCmdPort{
                in item fuelCmd:FuelCmd redefines pwrCmd;
            }
            port def FuelPort{
                out item fuel:Fuel;
            }
            port def DrivePwrPort{
                out torque:Torque;
            }
            port def ShaftPort_a;
            port def ShaftPort_b;
            port def ShaftPort_c;
            port def ShaftPort_d;
            port def DiffPort;
            port def AxlePort;
            port def AxleToWheelPort;
            port def WheelToAxlePort;
            port def WheelToRoadPort;

            port def LugNutCompositePort{
                port lugNutPort:LugNutPort [*];
            }
            port def ShankCompositePort{
                port shankPort:ShankPort [*];
            }
            port def LugNutPort{
                attribute threadDia;
                attribute threadPitch;
            }
            port def ShankPort{
                attribute threadDia;
                attribute threadPitch;   
                attribute shaftLength;
            }
            
            port def VehicleToRoadPort;
            port def ControlPort;
            port def CruiseControlPort:>ControlPort;
            port def SpeedSensorPort;
            port def SetSpeedPort;

            port def DriverCmdPort{
                out item driverCmd[*]:DriverCmd;
            }
            port def HandPort :> DriverCmdPort {
                out item ignitionCmd:IgnitionCmd subsets driverCmd;
                out item pwrCmd:PwrCmd subsets driverCmd;
            }  
        }
        package ItemDefinitions{
            item def PwrCmd{
                attribute throttleLevel:Real;
            }
            item def FuelCmd:>PwrCmd;
            item def Fuel{
                attribute fuelMass:>ISQ::mass;
            }
            item def SensedSpeed{
                attribute speed:>ISQ::speed;
            }
        }
        package SignalDefinitions{
            item def Cmd{
            }
            item def DriverCmd;
            item def IgnitionCmd:>DriverCmd{
                attribute ignitionOnOff:IgnitionOnOff;
            }
            item def EngineStatus;
            
            attribute def VehicleStartSignal;
            attribute def VehicleOnSignal;
            attribute def VehicleOffSignal;
            attribute def StartSignal;
            attribute def OffSignal;
            attribute def OverTemp;
            attribute def ReturnToNormal;
            attribute def SetSpeed:>Real;
        }
        package InterfaceDefinitions{
            interface def EngineToTransmissionInterface{
                end p1:DrivePwrPort;
                end p2:~DrivePwrPort;
                flow p1.torque to p2.torque;
            }
            interface def FuelInterface {
                end fuelOutPort:FuelPort;
                end fuelInPort:~FuelPort;
                flow of Fuel from fuelOutPort.fuel to fuelInPort.fuel;
            }
            
            interface def WheelFastenerInterface{
                end lugNutPort:LugNutPort;
                end shankPort:ShankPort;
                attribute maxTorque : Torque;
                constraint {lugNutPort.threadDia == shankPort.threadDia}
            }
            interface def WheelHubInterface{
                end lugNutCompositePort:LugNutCompositePort;
                end shankCompositePort:ShankCompositePort;
                interface wheelFastenerInterface:WheelFastenerInterface [5]
                    connect lugNutCompositePort.lugNutPort to shankCompositePort.shankPort;
            }
        }
        package AllocationDefinitions{
            allocation def LogicalToPhysical{
                end #logical logicalEnd;
                end #physical physicalEnd;
            }
        }
        package ActionDefinitions{
            action def ProvidePower {
                in item pwrCmd:PwrCmd;
                out wheelToRoadTorque:Torque[2];
            }
            action def GenerateTorque {
                in item fuelCmd:FuelCmd;
                out engineTorque:Torque;
            }
            action def AmplifyTorque {
                in engineTorque:Torque;
                out transmissionTorque:Torque;
            }
            action def TransferTorque {
                in transmissionTorque:Torque;
                out driveshaftTorque:Torque;
            }
            action def DistributeTorque {
                in driveshaftTorque:Torque;
                out wheelToRoadTorque:Torque[2];
            }
            action def PerformSelfTest;
            action def ApplyParkingBrake;
            action def SenseTemperature{
                out temp: ISQ::TemperatureValue;
            }
        }    
        package StateDefinitions {
            state def VehicleStates;
            state def ControllerStates;  
            state def CruiseControllerStates;
        }
        package RequirementDefinitions{
            requirement def MassRequirement{
                doc /*The actual mass shall be less than the required mass*/
                attribute massRequired:>ISQ::mass;
                attribute massActual:>ISQ::mass;
                require constraint {massActual<=massRequired}
            }
            requirement def ReliabilityRequirement{
                doc /*The actual reliability shall be greater than the required reliability*/
                attribute reliabilityRequired:Real;
                attribute reliabilityActual:Real;
                require constraint {reliabilityActual>=reliabilityRequired}
            }
            requirement def TorqueGenerationRequirement {
                doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
                subject generateTorque:ActionDefinitions::GenerateTorque;
            }
            requirement def DrivePowerOutputRequirement { 
                doc /* The engine shall provide a connection point to transfer torque to the transmission.*/
            }
            requirement def FuelEconomyRequirement {
                doc /* The vehicle shall maintain an average fuel economomy of at least x miles per gallon for the nominal 
                driving scenario */
                attribute actualFuelEconomy :> distancePerVolume;
                attribute requiredFuelEconomy :> distancePerVolume;
                require constraint {actualFuelEconomy >= requiredFuelEconomy}
            }
        }
        package AttributeDefinitions{
            public import ScalarValues::*;
            public import Quantities::*;
            public import MeasurementReferences::DerivedUnit;
            public import SIPrefixes::kilo;
            // Numerical Functions provides basic operators such as Sum expression
            public import NumericalFunctions::*;
            public import SI::*;
            public import USCustomaryUnits::*;
            alias Torque for ISQ::TorqueValue;
            
            enum def Colors {black;grey;red;}
            enum def DiameterChoices:>ISQ::LengthValue{
                enum = 60 [mm];
                enum = 80 [mm];
                enum = 100 [mm];
            }
            attribute cylinderDiameter: DiameterChoices = 80 [mm]; 
            enum def IgnitionOnOff {on;off;}
            enum def FuelKind {gas;diesel;}

            distancePerVolume :> scalarQuantities = distance / volume;
            timePerDistance :> scalarQuantities = time / distance;
            volumePerDistance :> scalarQuantities = volume / distance;
            volumePerTime :> scalarQuantities = volume / time;
            
            // kpl is approx .425 * mpg
            kpl : DerivedUnit = km / L;
            rpm : DerivedUnit = 1 / SI::min;
            kW : DerivedUnit = kilo * W;
            
        }
        package IndividualDefinitions{
            individual def VehicleRoadContext_1:>GenericContext::Context;
            individual def Vehicle_1:>Vehicle;
            individual def FrontAxleAssembly_1:>AxleAssembly;
            individual def FrontAxle_1:>FrontAxle;
            individual def Wheel_1:>Wheel;
            individual def Wheel_2:>Wheel;
            individual def RearAxleAssembly_1:>AxleAssembly;
            individual def Road_1:>Road;
        }
        package MetadataDefinitions { 
            public import AnalysisTooling::*;   
            metadata def Safety {
                attribute isMandatory : Boolean;
            }
            metadata def Security;
        }
        package KeyWord_MetadataDefinitions{
            public import Metaobjects::SemanticMetadata;
            
            // the following is used to define the key word failureMode
            state failureModes[*] nonunique;
            
            // with alias <fm>
            metadata def <fm> failureMode :> SemanticMetadata {
                :>> baseType = failureModes meta SysML::StateUsage;
            }
            
            occurrence logicalOccurrences [*] nonunique;
            
            metadata def <l> logical :> SemanticMetadata {
                :>> baseType = logicalOccurrences meta SysML::Usage;
            }
            
            occurrence physicalOccurrences [*] nonunique;
            
            metadata def <p> physical :> SemanticMetadata {
                :>> baseType = physicalOccurrences meta SysML::Usage;
            }  
        }
        package GenericContext {

            part def Context {
                attribute time:TimeValue;
                attribute spatialCF: CartesianSpatial3dCoordinateFrame[1] { :>> mRefs = (m, m, m); }
                attribute velocityCF: CartesianVelocity3dCoordinateFrame[1] = spatialCF/s;
                attribute accelarationCF: CartesianAcceleration3dCoordinateFrame[1] = velocityCF/s;
            }
        }
    }

    package VehicleLogicalConfiguration{
        package PartsTree{
            #logical part vehicleLogical:Vehicle{
                part torqueGenerator:TorqueGenerator{
                    action generateTorque;
                }
                part electricalGenerator:ElectricalGenerator{
                    action generateElectricity;
                }
                part steeringSystem:SteeringSubsystem;
                part brakingSubsystem:BrakingSubsystem;
            }
        }
    }
    package VehicleLogicalToPhysicalAllocation{
        public import VehicleConfigurations::VehicleConfiguration_b::PartsTree::**;
        public import VehicleLogicalConfiguration::PartsTree::*;

        allocation vehicleLogicalToPhysicalAllocation:LogicalToPhysical
            allocate vehicleLogical to vehicle_b{
                allocate vehicleLogical.torqueGenerator to vehicle_b.engine{
                    allocate vehicleLogical.torqueGenerator.generateTorque to vehicle_b.engine.generateTorque;
                }
                allocate vehicleLogical.electricalGenerator to vehicle_b.engine{
                    allocate vehicleLogical.electricalGenerator.generateElectricity to vehicle_b.engine.alternator.generateElectricity;
                }
            }
    } 
    package VehicleConfigurations{
        package VehicleConfiguration_a{
            package PartsTree{
                part vehicle_a:Vehicle{
                    attribute mass redefines Vehicle::mass=dryMass+cargoMass+fuelTank.fuel.fuelMass;
                    attribute dryMass redefines Vehicle::dryMass=sum(partMasses);
                    attribute redefines Vehicle::cargoMass=0 [kg];
                    attribute partMasses [*] nonunique :>ISQ::mass;
                    part fuelTank:FuelTank{
                        attribute redefines mass=75[kg];
                        ref item redefines fuel{
                            attribute redefines fuelMass=50[kg];
                        }   
                    }
                    part frontAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=800[kg];
                        part frontAxle:Axle;
                        part frontWheels:Wheel[2];
                    }
                    part rearAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=875[kg];
                        attribute driveTrainEfficiency:Real = 0.6;
                        part rearAxle:Axle;
                        part rearWheels:Wheel[2]{
                            attribute redefines diameter;
                        }
                    }
                }
            }
            package ActionTree{  
            }
            package Requirements{
            }
        }
        package VehicleConfiguration_b{
            //Shapes library for simple geometry
            public import ShapeItems::Box;
            public import ParametersOfInterestMetadata::mop;
            public import ModelingMetadata::*; // incudes status info
            
            package PartsTree{
                part vehicle_b : Vehicle{
                    #mop attribute mass redefines mass=dryMass+cargoMass+fuelTank.fuel.fuelMass;
                    attribute dryMass redefines dryMass=sum(partMasses);
                    attribute redefines cargoMass default 0 [kg];
                    attribute partMasses=(fuelTank.mass,frontAxleAssembly.mass,rearAxleAssembly.mass,engine.mass,transmission.mass,driveshaft.mass);
                    attribute avgFuelEconomy :> distancePerVolume;
                    port fuelCmdPort: FuelCmdPort redefines pwrCmdPort {
                        in item fuelCmd redefines pwrCmd;
                    }
                    port setSpeedPort:~SetSpeedPort;
                    port vehicleToRoadPort redefines vehicleToRoadPort{
                        port wheelToRoadPort1:WheelToRoadPort;
                        port wheelToRoadPort2:WheelToRoadPort;
                    }
                    perform ActionTree::providePower redefines providePower;
                    perform ActionTree::performSelfTest redefines performSelfTest;
                    perform ActionTree::applyParkingBrake redefines applyParkingBrake;
                    perform ActionTree::senseTemperature redefines senseTemperature;
                    exhibit state vehicleStates redefines vehicleStates;
                    
                    // Example vehicle with simple enveloping shape that is a solid 
                    item :> envelopingShapes : Box[1] {
                        length1:>> length = 4800 [mm];
                        width1:>> width = 1840 [mm];
                        height1:>> height = 1350 [mm];
                    }
                    
                    part fuelTank:FuelTank{
                        attribute redefines mass=75[kg];
                        ref item redefines fuel{
                            attribute redefines fuelMass=60[kg];
                        }
                        attribute redefines fuelMassMax=60 [kg];
                    }
                    part frontAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=800[kg];
                        port shaftPort_d:ShaftPort_d;
                        part frontAxle:FrontAxle;
                        part frontWheels:Wheel[2];
                    }
                    
                    part rearAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=875[kg];
                        attribute driveTrainEfficiency:Real = 0.6;
                        port shaftPort_d:ShaftPort_d;
                        perform providePower.distributeTorque;
                        part rearWheel1:Wheel{
                            attribute redefines diameter;
                            port wheelToRoadPort:WheelToRoadPort;
                            port lugNutCompositePort :>> lugNutCompositePort{
                                port lugNutPort :>> lugNutPort [5];
                            }
                        }
                        part rearWheel2:Wheel{
                            attribute redefines diameter;
                            port wheelToRoadPort:WheelToRoadPort;
                            port lugNutCompositePort :>> lugNutCompositePort{
                                port lugNutPort :>> lugNutPort [5];
                            }
                        }
                        part differential:Differential{
                            port shaftPort_d:ShaftPort_d;
                            port leftDiffPort:DiffPort;
                            port rightDiffPort:DiffPort;
                        }
                        part rearAxle{
                            part leftHalfAxle:HalfAxle{
                                port leftAxleToDiffPort:AxlePort;
                                port shankCompositePort :>> shankCompositePort{
                                    port shankPort :>> shankPort [5];
                                }
                            }
                            part rightHalfAxle:HalfAxle{
                                port rightAxleToDiffPort:AxlePort;
                                port shankCompositePort :>> shankCompositePort {
                                    port shankPort :>> shankPort [5];
                                }
                            }
                        }
                        
                        bind shaftPort_d=differential.shaftPort_d;
                        connect differential.leftDiffPort to rearAxle.leftHalfAxle.leftAxleToDiffPort;
                        connect differential.rightDiffPort to rearAxle.rightHalfAxle.rightAxleToDiffPort;
                        
                        interface wheelToleftHalAxleInterface:WheelHubInterface 
                            connect [1] rearWheel1.lugNutCompositePort to [1] rearAxle.leftHalfAxle.shankCompositePort;
                        interface wheelTorightHalAxleInterface:WheelHubInterface
                            connect [1] rearWheel2.lugNutCompositePort to [1] rearAxle.rightHalfAxle.shankCompositePort;
                        
                    }
                    part starterMotor:StarterMotor;
                    part engine:Engine{
                        perform providePower.generateTorque redefines generateTorque;            
                        part cylinders:Cylinder[4..6];
                        part alternator{
                            action generateElectricity;
                        }
                        satisfy Requirements::engineSpecification by vehicle_b.engine{
                            requirement torqueGenerationRequirement :>> torqueGenerationRequirement{
                                subject generateTorque redefines generateTorque = vehicle_b.engine.generateTorque;
                            }
                            requirement drivePowerOuputRequirement :>> drivePowerOutputRequirement{
                                port torqueOutPort redefines torqueOutPort=vehicle_b.engine.drivePwrPort;
                            }
                        } 
                    }
                    part transmission:Transmission{
                        attribute mass :> ISQ::mass=100[kg];
                        port shaftPort_a:ShaftPort_a;
                        perform providePower.amplifyTorque;
                    }
                    part driveshaft:Driveshaft{
                        attribute mass :> ISQ::mass=100[kg];
                        port shaftPort_b:ShaftPort_b;
                        port shaftPort_c:ShaftPort_c;
                        perform providePower.transferTorque;
                    }
                    part vehicleSoftware:VehicleSoftware{
                        part vehicleController: VehicleController {
                            exhibit state controllerStates redefines controllerStates;
                            part cruiseController:CruiseController;
                        }
                    }
                    part speedSensor:SpeedSensor;
                    
                    // parts in bodyAssy and interioer are marked as safety or security features
                    part bodyAssy:BodyAssy{
                        part body:Body{
                            attribute :>> color = Colors::red;  
                        }
                        part bumper {@Safety{isMandatory = true;}}
                        part keylessEntry {@Security;}
                    }
                    part interior {
                        part alarm {@Security;}
                        part seatBelt[2] {@Safety{isMandatory = true;}}
                        part frontSeat[2];
                        part driverAirBag {@Safety{isMandatory = false;}}
                    }
                    
                    //connections
                    bind engine.fuelCmdPort=fuelCmdPort;

                    interface engineToTransmissionInterface:EngineToTransmissionInterface
                        connect engine.drivePwrPort to transmission.clutchPort;
                
                    interface fuelInterface:FuelInterface
                        connect fuelTank.fuelOutPort to engine.fuelInPort;

                    allocate ActionTree::providePower.generateToAmplify to engineToTransmissionInterface;
                    
                    bind engine.ignitionCmdPort=ignitionCmdPort;
                    connect starterMotor.gearPort to engine.flyWheelPort;
                    connect vehicleSoftware.vehicleController.controlPort to engine.engineControlPort;
                    bind vehicle_b.setSpeedPort = vehicleSoftware.vehicleController.cruiseController.setSpeedPort;
                    connect speedSensor.speedSensorPort to vehicleSoftware.vehicleController.cruiseController.speedSensorPort;
                    bind vehicleSoftware.vehicleController.cruiseController.cruiseControlPort = vehicleSoftware.vehicleController.controlPort;
                    connect transmission.shaftPort_a to driveshaft.shaftPort_b; 
                    connect driveshaft.shaftPort_c to rearAxleAssembly.shaftPort_d;
                    bind rearAxleAssembly.rearWheel1.wheelToRoadPort=vehicleToRoadPort.wheelToRoadPort1;
                    bind rearAxleAssembly.rearWheel2.wheelToRoadPort=vehicleToRoadPort.wheelToRoadPort2;
                    
                    satisfy Requirements::vehicleSpecification by vehicle_b{
                        requirement vehicleMassRequirement:>>vehicleMassRequirement{
                            attribute redefines massActual=vehicle_b.mass;
                            attribute redefines fuelMassActual = vehicle_b.fuelTank.fuel.fuelMass;
                        }
                    }
                }
            }
            package ActionTree{
                action providePower:ProvidePower{
                    in item fuelCmd:FuelCmd redefines pwrCmd;
                    out wheelToRoadTorque redefines wheelToRoadTorque [2] = distributeTorque.wheelToRoadTorque;
                    action generateTorque:GenerateTorque {
                        in item = providePower.fuelCmd;
                    }
                    action amplifyTorque:AmplifyTorque;
                    action transferTorque:TransferTorque;
                    action distributeTorque:DistributeTorque;
                    
                    //named flow
                    flow generateToAmplify from generateTorque.engineTorque to amplifyTorque.engineTorque;
                    //unnamed flows
                    flow amplifyTorque.transmissionTorque to transferTorque.transmissionTorque;
                    flow transferTorque.driveshaftTorque to distributeTorque.driveshaftTorque;
                }
                action performSelfTest: PerformSelfTest;
                action applyParkingBrake: ApplyParkingBrake;
                action senseTemperature: SenseTemperature;
            }                   
            package DiscreteInteractions{
                package Sequence{
                    part def Driver{
                        port p1;
                        port p2;
                    }

                    part part0{
                        perform action startVehicle{
                            action turnVehicleOn send ignitionCmd via driver.p1{
                                in ignitionCmd:IgnitionCmd;
                            }
                            action trigger1 accept ignitionCmd:IgnitionCmd via vehicle.ignitionCmdPort;
                            flow of IgnitionCmd from trigger1.ignitionCmd to startEngine.ignitionCmd;
                            action startEngine{
                                in item ignitionCmd:IgnitionCmd; 
                                out item es:EngineStatus;
                            }
                            flow of EngineStatus from startEngine.es to sendStatus.es;
                            action sendStatus send es via vehicle.statusPort{
                                in es:EngineStatus;
                            }
                            action trigger2 accept es:EngineStatus via driver.p2;
                        }
                        part driver : Driver {
                            perform startVehicle.turnVehicleOn;
                            perform startVehicle.trigger2;
                            event occurrence driverReady;
                        }
                        part vehicle : Vehicle {
                            perform startVehicle.trigger1;
                            perform startVehicle.sendStatus;
                            event occurrence doorClosed;
                        }
                        first vehicle.doorClosed then driver.driverReady;
                        message of ignitionCmd:IgnitionCmd from driver.turnVehicleOn to vehicle.trigger1;  
                        message of es:EngineStatus from vehicle.sendStatus to driver.trigger2;
                    }
                }
                occurrence CruiseControl1{
                    part vehicle_b:>PartsTree::vehicle_b{
                        port redefines setSpeedPort{
                            event occurrence setSpeedReceived;
                        }
                        part redefines speedSensor{
                            port redefines speedSensorPort{
                                event occurrence sensedSpeedSent;
                            }
                        }
                        part redefines vehicleSoftware{
                            part redefines vehicleController{
                                part redefines cruiseController{
                                    port redefines setSpeedPort{
                                        //analagous to gate: event occurrence bound but may not need this since the port is bound
                                        event occurrence setSpeedReceived = vehicle_b.setSpeedPort.setSpeedReceived;
                                    }
                                    port redefines speedSensorPort{
                                        event occurrence sensedSpeedReceived;
                                    }
                                    port redefines cruiseControlPort{
                                        event occurrence fuelCmdSent;
                                    }
                                }
                            }
                        }
                        part redefines engine{
                            port redefines fuelCmdPort{
                                event occurrence fuelCmdReceived;
                            }
                        }
                        message sendSensedSpeed of SensedSpeed
                            from speedSensor.speedSensorPort.sensedSpeedSent to vehicleSoftware.vehicleController.cruiseController.speedSensorPort.sensedSpeedReceived;
                        message sendFuelCmd of FuelCmd
                            from vehicleSoftware.vehicleController.cruiseController.cruiseControlPort.fuelCmdSent to engine.fuelCmdPort.fuelCmdReceived;
                    }
                }
                occurrence CruiseControl2{
                    part vehicle_b:>PartsTree::vehicle_b{
                        port redefines setSpeedPort{
                            event occurrence setSpeedReceived;
                        }
                        part redefines speedSensor{
                            port redefines speedSensorPort{
                                event sendSensedSpeed.sourceEvent;
                            }
                        }
                        part redefines vehicleSoftware{
                            part redefines vehicleController{
                                part redefines cruiseController{
                                    port redefines setSpeedPort{
                                        //analagous to gate: event occurrence bound but may not need this since the port is bound
                                        event occurrence setSpeedReceived = vehicle_b.setSpeedPort.setSpeedReceived;
                                    }
                                    port redefines speedSensorPort{
                                        event occurrence setSpeedReceived=setSpeedPort.setSpeedReceived;
                                        then event sendSensedSpeed.targetEvent;
                                    }
                                    port redefines cruiseControlPort{             
                                        event sendFuelCmd.sourceEvent;
                                    }
                                }
                            }
                        }
                        part redefines engine{
                            port redefines fuelCmdPort{
                                event sendFuelCmd.targetEvent;
                            }
                        }
                        message sendSensedSpeed of SensedSpeed;
                        message sendFuelCmd of FuelCmd;
                    }
                }
            }
            package Requirements{
                public import RequirementDerivation::*;
                public import ModelingMetadata::*; // incudes status info
                item marketSurvey;
                dependency from vehicleSpecification to marketSurvey;
                
                requirement vehicleSpecification{
                    subject vehicle:Vehicle;
                    requirement <'1'> vehicleMassRequirement: MassRequirement {
                        doc /* The total mass of the vehicle shall be less than or equal to the required mass.
                        Assume total mass includes a full tank of gas of 60 kg*/
                        attribute redefines massRequired=2000 [kg];                     
                        attribute redefines massActual default vehicle.dryMass + fuelMassActual;
                        attribute fuelMassActual:>ISQ::mass;
                        attribute fuelMassMax:>ISQ::mass = 60 [kg];
                        assume constraint {fuelMassActual==fuelMassMax}
                    }
                    
                    allocate vehicleMassRequirement to PartsTree::vehicle_b.mass;
                    
                    requirement <'2'> vehicleFuelEconomyRequirements{
                        doc /* fuel economy requirements group */
                        attribute assumedCargoMass:>ISQ::mass;
                        requirement <'2_1'> cityFuelEconomyRequirement:FuelEconomyRequirement{
                            redefines requiredFuelEconomy= 10 [km / L];
                            assume constraint {assumedCargoMass<=500 [kg]}
                        }
                        requirement <'2_2'> highwayFuelEconomyRequirement:FuelEconomyRequirement{
                            redefines requiredFuelEconomy= 12.75 [km / L];
                            assume constraint {assumedCargoMass<=500 [kg]}
                            
                            //StatusInfo is contained in ModelingMetadata library
                            // StatusKind has values for open, closed, tbd, tbr, tbd
                            @StatusInfo {
                                status = StatusKind::closed;     
                                originator = "Bob";
                                owner = "Mary";
                            }
                        }
                    }
                }
                requirement engineSpecification {
                    subject engine1:Engine;
                    requirement <'1'> engineMassRequirement: MassRequirement {
                        doc /* The total mass of the engine shall be less than or equal to the required mass.*/
                        attribute redefines massRequired=200 [kg];                     
                        attribute redefines massActual = engine1.mass;
                    }
                    requirement torqueGenerationRequirement : TorqueGenerationRequirement{
                        subject generateTorque default engine1.generateTorque;
                    }

                    requirement drivePowerOutputRequirement : DrivePowerOutputRequirement{
                        port torqueOutPort{
                            out torque:Torque;
                        }
                    }
                }
                // the engine mass requirement is derived from the vehicle mass requirement
                #derivation connection {
                    end #original ::> vehicleSpecification.vehicleMassRequirement;
                    end #derive ::> engineSpecification.engineMassRequirement;
                }

            }
        }    
        package Engine4Cyl_Variant{
            public import ModelingMetadata::*; // incudes refinement
            part engine:Engine{
                part cylinders:Cylinder[4..8] ordered;
            }
            part engine4Cyl:>engine{
                part redefines cylinders [4];
                part cylinder1 subsets cylinders[1];
                part cylinder2 subsets cylinders[1];
                part cylinder3 subsets cylinders[1];
                part cylinder4 subsets cylinders[1];
            }
            #refinement dependency engine4Cyl to VehicleConfiguration_b::PartsTree::vehicle_b::engine;
        }
        package WheelHubAssemblies{
            // alternative 1 - w/o explicit nesxted interfaces
            part wheelHubAssy1{
                part wheel1:Wheel{
                    port :>>lugNutCompositePort:LugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5];
                    }
                }
                part hub1:Hub{
                    port :>> shankCompositePort:ShankCompositePort {
                        port shankPort :>> shankPort [5];
                    }
                }
                interface wheelHubInterface:WheelHubInterface
                    connect [1] wheel1.lugNutCompositePort to [1] hub1.shankCompositePort;
            }
            // alternative 2 - w multiple nesxted interfaces
            part wheelHubAssy2{
                part wheel1:Wheel{
                    port :>>lugNutCompositePort:LugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5];
                    }
                }
                part hub1:Hub{
                    port :>> shankCompositePort:ShankCompositePort {
                        port shankPort :>> shankPort [5];
                    }
                }
                interface wheelHubInterface:WheelHubInterface
                    connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort {
                        interface wheelFastenerInterface1 :> wheelFastenerInterface
                            connect [5] lugNutPort ::> lugNutCompositePort.lugNutPort to [5] shankPort ::> shankCompositePort.shankPort;
                        }
            }
            // alternative 3 - w explicit nesxted interfaces
            part wheelHubAssy3{
                part wheel1:Wheel{
                    port lugNutCompositePort :>> lugNutCompositePort {
                        port lugNutPort [5] :>> lugNutPort {
                            attribute :>> threadDia = 14 [mm];
                            attribute :>> threadPitch = 1.5 [mm];
                        }
                        port lugNutPort1 [1] :> lugNutPort;
                        port lugNutPort2 [1] :> lugNutPort;
                        port lugNutPort3 [1] :> lugNutPort;
                    }
}
                part hub1:Hub{
                    port shankCompositePort :>> shankCompositePort {
                        port shankPort [5] :>> shankPort {
                            attribute :>> threadDia = 14 [mm];
                            attribute :>> threadPitch = 1.5 [mm];
                            attribute :>> shaftLength = 70 [mm];
                        }
                        port shankPort1 [1] :> shankPort;
                        port shankPort2 [1] :> shankPort;
                        port shankPort3 [1] :> shankPort;
                    }
}
                interface wheelHubInterface:WheelHubInterface
                    connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort {
                        interface wheelFastenerInterface1 :> wheelFastenerInterface
                            connect lugNutPort ::> lugNutCompositePort.lugNutPort1 to shankPort ::> shankCompositePort.shankPort1 {
                                attribute :>> maxTorque = 90 * 1.356 [N*m];
                        }
                        interface wheelFastenerInterface2 :> wheelFastenerInterface
                            connect lugNutPort ::> lugNutCompositePort.lugNutPort2 to shankPort ::> shankCompositePort.shankPort2 {
                                attribute :>> maxTorque = 90 * 1.356 [N*m];
                        }
                        interface wheelFastenerInterface3 :> wheelFastenerInterface
                            connect lugNutPort ::> lugNutCompositePort.lugNutPort3 to shankPort ::> shankCompositePort.shankPort3 {
                                attribute :>> maxTorque = 90 * 1.356 [N*m];
                        }
                }
            }
        }
    }
    package VehicleAnalysis{
        public import RiskMetadata::*;
        public import RiskLevelEnum::*;
        // recursive public import uses double asterisk **
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        package FuelEconomyAnalysisModel{
            public import SampledFunctions::SampledFunction;
              
            /*
            This analysis model was provided by Hisashi Miyashita on January 27, 2021
              We use the simplest fuel consumption analysis model introduced in:
              Akcelik, R. "Fuel efficiency and other objectives in traffic system management." Traffic Engineering and Control 22.2 (1981): 54-65. 

              Fuel consumption rate f can be decomposed to:
              f = f_a + f_b * tpd_avg,
              where tpd_avg is average interrupted travel time per unit distance, actually the inverse of the average velocity [t/km];
              f_a is the best fuel consumption per distance; and
              f_b is the additional fuel consumption per distance and average travel time, which can be regarded as the idling fuel consumption.
              Approximately, it is proportional to engine displacement and it ranges from 0.5 to 0.6 [l/hour/litre of engine displacement]
              according to:
              Review of the Incidence, Energy Use and Costs of Passenger Vehicle Idling; Gordon W. Taylor, P.Eng. Prepared for the Office of Energy Efficiency, Natural Resources Canada, 2003

              We assume f_a can be approximated to
              fuel_consumption / distance = BSFC * SGG * required_power_avg * tpd_avg,
              where required_power_avg is the required power, and it can be approximately derived from:
                  total_energy == P_req * tpd_avg * distance == 1/2 * mass / tpd_avg^2
              This part is computed with BestFuelConsumptionPerDistance calc def.

              BSFC means Brake-Specific Fuel Consumption, defined as gram/power.  SGG is the specific gravity of gasoline.
              The high octane gasoline is about 0.76[l/kg].
            */
            
            attribute def Scenario :> SampledFunction {
                attribute wayPoint[1..*] {
                    attribute elapseTime[1] :> ISQ::time;
                    attribute position[1] :> ISQ::distance;
                }
            }
            
            calc def FuelConsumption {
                in bestFuelConsumption: Real;
                in idlingFuelConsumption: Real; 
                in tpd_avg:>timePerDistance;
                attribute f = bestFuelConsumption + idlingFuelConsumption * tpd_avg;
                return dpv :> distancePerVolume = 1/f;
            }
            
            calc def AverageTravelTimePerDistance {
                in scenario: Scenario;
                return tpd_avg:>timePerDistance;
            }
            calc def TraveledDistance {
                in scenario: Scenario;
                return distance:> length;
            }
            calc def IdlingFuelConsumptionPerTime {
                in engine:Engine;
                attribute idlingFuelConsumptionPerDisplacement: Real = 0.5;
                return f_a : Real = engine.displacement * idlingFuelConsumptionPerDisplacement;
            }

            attribute specificGravityOfGasoline: Real = 0.76;
            calc def BestFuelConsumptionPerDistance {
                in mass: MassValue;
                in bsfc: Real;
                in tpd_avg:> timePerDistance;
                in distance:>length;
                attribute required_power_avg:> ISQ::power;
                constraint {required_power_avg == 1/2 * mass * tpd_avg **(-3) / distance}
                return f_b : Real = bsfc * specificGravityOfGasoline * required_power_avg * tpd_avg;
            }

            calc def ComputeBSFC{
                in engine: Engine;
                return : Real;
            }

            analysis fuelEconomyAnalysis  {    
                subject = vehicle_b; 
                
                objective fuelEconomyAnalysisObjective {
                    doc /*estimate the vehicle fuel economy*/
                    require vehicleSpecification.vehicleFuelEconomyRequirements;
                }
                
                in attribute scenario: Scenario;
                // define a series of waypoints
                
                attribute distance = TraveledDistance(scenario);
                attribute tpd_avg = AverageTravelTimePerDistance(scenario);
                attribute bsfc = ComputeBSFC(vehicle_b.engine);
                attribute f_a = BestFuelConsumptionPerDistance(vehicle_b.mass, bsfc, tpd_avg, distance);
                attribute f_b = IdlingFuelConsumptionPerTime(vehicle_b.engine);

                return attribute calculatedFuelEconomy:>distancePerVolume=FuelConsumption(f_a, f_b, tpd_avg);
            }
        }
        package ElectricalPowerAnalysis{
        }
        package ReliabilityAnalyis{
        }
        package VehicleTradeOffAnalysis{
            /* The following example provides the rationale for selecting the engine4cyl. 
            The rationale and risk are contained in a metadata library. */
            
            @Rationale about engineTradeOffAnalysis::vehicle_b_engine4cyl{
                explanation = VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis;          
                text = "the engine4cyl was evaluated to have a higher objective function compared to the engine6cyl based on the trade-off analyiss"; 
            }
            
            // The following risk for the engine4cyl could have been included as part of the objective evaluaiton criteria
            
            @Risk about engineTradeOffAnalysis::vehicle_b_engine4cyl {
                totalRisk = medium;
                technicalRisk = medium;
                scheduleRisk = medium;
                costRisk = RiskLevelEnum::low;
            }
            @Risk about engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::fuelEfficiency {
                technicalRisk {
                    probability = 0.3;
                    impact = 0.5;
                }
            }
            
                
            public import TradeStudies::*;
            //evaluation function with criterion engine mass, engine power, and engine cost
            calc def EngineEvaluation {
                in engineMass:>ISQ::mass;
                in enginePower:>ISQ::power; 
                in engineFuelEfficiency:Real;
                in engineCost:Real;
                return eval:Real;
            }
            calc def EngineEvaluation_4cyl {
                in engineMass:>ISQ::mass;
                in enginePower:>ISQ::power;
                in engineFuelEfficiency:Real;
                in engineCost:Real;
                return eval:Real;
            }
            calc def EngineEvaluation_6cyl {
                in engineMass:>ISQ::mass;
                in enginePower:>ISQ::power;
                in engineFuelEfficiency:Real;
                in engineCost:Real;
                return eval:Real;
            }
            analysis engineTradeOffAnalysis:TradeStudy{
                subject vehicleAlternatives[2]:>vehicle_b;   
                
                part vehicle_b_engine4cyl:>vehicleAlternatives{   
                    part engine redefines engine{
                        part cylinders :>> cylinders [4];
                        attribute mass redefines mass=180 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 180 [W];
                        attribute fuelEfficiency redefines fuelEfficiency=.6;
                        attribute cost redefines cost = 1000;                     
                    }
                }
                part vehicle_b_engine6cyl:>vehicleAlternatives{   
                    part engine redefines engine{  
                        part cylinders redefines cylinders [6];
                        attribute mass redefines mass=220 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 220 [W];
                        attribute fuelEfficiency redefines fuelEfficiency=.5;
                        attribute cost redefines cost = 1500;
                    }
                }
                
                objective :MaximizeObjective;
                    /*Select vehicle alternative with the engine whose evaluation function returns the max value*/
                
                calc :> evaluationFunction{
                    in part vehicle:>vehicle_b_engine4cyl;
                    return attribute eval:Real=EngineEvaluation_4cyl (vehicle.engine.mass, vehicle.engine.peakHorsePower, vehicle.engine.fuelEfficiency, vehicle.engine.cost); 
                }
                calc :> evaluationFunction{
                    in part vehicle:>vehicle_b_engine6cyl;
                    return attribute eval:Real=EngineEvaluation_6cyl (vehicle.engine.mass, vehicle.engine.peakHorsePower, vehicle.engine.fuelEfficiency, vehicle.engine.cost); 
                }                                                  
                return part selectedVehicle:>vehicle_b;
            }
        }
    }
    package VehicleVerification{
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        public import VerificationCaseDefinitions::*;
        public import VerificationCases1::*;
        // the following is a model library which contains VerdictKind
        public import VerificationCases::*;
        public import VerificationSystem::*;
        package VerificationCaseDefinitions{
            verification def MassTest;
            verification def AccelerationTest;
            verification def ReliabilityTest;
        }
        package VerificationCases1{
            verification massTests:MassTest {
                subject vehicle_uut :> vehicle_b;
                actor vehicleVerificationSubSystem_1 = verificationContext.massVerificationSystem;
                objective {
                    verify vehicleSpecification.vehicleMassRequirement{
                        redefines massActual=weighVehicle.massMeasured;
                    }
                }     
                // method kinds are test, demo, analyze, should also include inspection, similarity
               @ VerificationMethod{
                    kind = (VerificationMethodKind::test, VerificationMethodKind::analyze);
                }
                action weighVehicle {
                    out massMeasured:>ISQ::mass;
                }
                then action evaluatePassFail {
                    in massMeasured:>ISQ::mass;
                    out verdict = PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut));
                }
                flow from weighVehicle.massMeasured to evaluatePassFail.massMeasured;
                return :>> verdict = evaluatePassFail.verdict;
            }
        }
        package VerificationSystem{
            part verificationContext{
                perform massTests;
                part vehicle_UnitUnderTest :> vehicle_b;
                part massVerificationSystem{
                    part scale{
                        perform massTests.weighVehicle;
                    }
                    part operator{
                        perform massTests.evaluatePassFail;
                    }
                }
            }
        }
    }
    package VehicleIndividuals{
        individual a:VehicleRoadContext_1{
            timeslice t0_t2_a{
                snapshot t0_a {             
                    attribute t0 redefines time=0 [s];
                    snapshot t0_r:Road_1{
                        :>>Road::incline =0;
                        :>>Road::friction=.1;
                    }
                    snapshot t0_v:Vehicle_1{
                        :>>Vehicle::position=0 [m];
                        :>>Vehicle::velocity=0 [m];
                        :>>Vehicle::acceleration=1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t0_fa:FrontAxleAssembly_1{
                            snapshot t0_leftFront:Wheel_1;
                            snapshot t0_rightFront:Wheel_2;
                        }
                    }
                }
                snapshot t1_a{
                    attribute t1 redefines time=1 [s];
                    snapshot t1_r:Road_1{
                        :>>Road::incline =0;
                        :>>Road::friction=.1;
                    }
                    snapshot t1_v:Vehicle_1{
                        :>>Vehicle::position=.98 [m];
                        :>>Vehicle::velocity=1.96 [m/s];
                        :>>Vehicle::acceleration=1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t1_fa:FrontAxleAssembly_1{
                            snapshot t1_leftFront:Wheel_1;
                            snapshot t1_rightFront:Wheel_2;
                        }
                    }
                }
                snapshot t2_a{
                    attribute t2 redefines time=2 [s];
                    snapshot t2_r:Road_1{
                        :>>Road::incline =0;
                        :>>Road::friction=.1;
                    }
                    snapshot t2_v:Vehicle_1{
                        :>>Vehicle::position=3.92 [m];
                        :>>Vehicle::velocity=3.92 [m/s];
                        :>>Vehicle::acceleration=1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t2_fa:FrontAxleAssembly_1{
                            snapshot t2_leftFront:Wheel_1;
                            snapshot t2_rightFront:Wheel_2;
                        }
                    }
                }
            }
        }
    }
    package MissionContext{
        /* Define mission context with mission use cases for vehicle_b */
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        public import ParametersOfInterestMetadata::moe;
        public import TransportPassengerScenario::*;
        package ContextDefinitions{
            part def MissionContext:>GenericContext::Context;
            part def Road;
            part def Driver{
                port handPort:HandPort{
                }
                exhibit state driverStates{
                    state initial;
                    state wait;
                    transition initial then wait;
                    //ignition on
                    transition 'wait-wait-1'
                        first wait
                        do send new IgnitionCmd (ignitionOnOff=IgnitionOnOff::on) via handPort
                        then wait;
                    // ignition off
                    transition 'wait-wait-2'
                        first wait
                        do send new IgnitionCmd (ignitionOnOff=IgnitionOnOff::off) via handPort
                        then wait;
                }
            }
            part def Passenger;
            
            requirement transportRequirements;
            use case def TransportPassenger{
                objective TransportObjective {
                    doc /*deliver passenger to destination safely, comfortably, and within acceptable time*/
                    require transportRequirements;
                }
                subject vehicle:Vehicle;
                actor environment;
                actor road;
                actor driver;
                actor passenger [0..4];
                include use case getInVehicle_a:>getInVehicle [1..5];
                include use case getOutOfVehicle_a:>getOutOfVehicle [1..5];
            }
            
            use case getInVehicle:GetInVehicle {
                action unlockDoor_in [0..1];
                then action openDoor_in;
                then action enterVehicle;
                then action closeDoor_in;
            }
            use case def GetInVehicle{
                subject vehicle:Vehicle;
                actor driver [0..1];
                actor passenger [0..1];
                assert constraint {driver != null xor passenger != null}
            }

            use case getOutOfVehicle:GetOutOfVehicle {
                action openDoor_out;
                then action exitVehicle;
                then action closeDoor_out;
                then action lockDoor_out;
            }
            use case def GetOutOfVehicle{
                subject vehicle:Vehicle;
                actor driver [0..1];
                actor passenger [0..1];
                assert constraint {driver != null xor passenger != null}
            }
        }
        package TransportPassengerScenario{
            public import ContextDefinitions::TransportPassenger;
            
            // this version uses nesting vs fork and join for concurrent actions
            use case transportPassenger:TransportPassenger{
                first start; 
                then action a{
                    action driverGetInVehicle subsets getInVehicle_a[1];
                    action passenger1GetInVehicle subsets getInVehicle_a[1];
                }
                then action trigger accept ignitionCmd:IgnitionCmd;
                then action b{
                    action driveVehicleToDestination;
                    action providePower;   
                }
                then action c{
                    action driverGetOutOfVehicle subsets getOutOfVehicle_a[1];
                    action passenger1GetOutOfVehicle subsets getOutOfVehicle_a[1];
                }
                then done;
            }
            
            
            //this version uses forks and joins
            use case transportPassenger_1:TransportPassenger{
                // declare actions
                action driverGetInVehicle subsets getInVehicle_a[1];
                action passenger1GetInVehicle subsets getInVehicle_a[1];
                action driverGetOutOfVehicle subsets getOutOfVehicle_a[1];
                action passenger1GetOutOfVehicle subsets getOutOfVehicle_a[1];
                action driveVehicleToDestination;
                action providePower;
                item def VehicleOnSignal;
                join join1;
                join join2;
                join join3;
                action trigger accept ignitionCmd:IgnitionCmd;
                
                // define control flow
                first start;               
                then fork fork1;
                    then driverGetInVehicle;
                    then passenger1GetInVehicle;
                first driverGetInVehicle then join1;
                first passenger1GetInVehicle then join1;
                first join1 then trigger;
                first trigger then fork2;
                //succession trigger if trigger.ignitionCmd.ignitionOnOff==IgnitionOnOff::on then fork2;
                
                fork fork2;
                    then driveVehicleToDestination;
                    then providePower;
                first driveVehicleToDestination then join2;
                first providePower then join2;
                first join2 then fork3;

                fork fork3; 
                    then driverGetOutOfVehicle;
                    then passenger1GetOutOfVehicle;
                first driverGetOutOfVehicle then join3;
                first passenger1GetOutOfVehicle then join3;

                first join3 then done;
            }
        }
        
        part missionContext:ContextDefinitions::MissionContext{
            #moe attribute transportTime :> ISQ::time;
            perform transportPassenger;
            // bind parts to actors of use case
            part road:ContextDefinitions::Road = transportPassenger.road;
            part driver:ContextDefinitions::Driver = transportPassenger.driver{
                perform transportPassenger.a.driverGetInVehicle.unlockDoor_in;
                perform transportPassenger.a.driverGetInVehicle.openDoor_in;
                perform transportPassenger.a.driverGetInVehicle.enterVehicle; 
                perform transportPassenger.a.driverGetInVehicle.closeDoor_in;
                perform transportPassenger.c.driverGetOutOfVehicle.openDoor_out;
                perform transportPassenger.c.driverGetOutOfVehicle.exitVehicle; 
                perform transportPassenger.c.driverGetOutOfVehicle.closeDoor_out;
                perform transportPassenger.c.driverGetOutOfVehicle.lockDoor_out;
                perform transportPassenger.b.driveVehicleToDestination;
            }
            part passenger1:ContextDefinitions::Passenger = transportPassenger.passenger {
                perform transportPassenger.a.passenger1GetInVehicle.unlockDoor_in;
                perform transportPassenger.a.passenger1GetInVehicle.openDoor_in;
                perform transportPassenger.a.passenger1GetInVehicle.enterVehicle; 
                perform transportPassenger.a.passenger1GetInVehicle.closeDoor_in;
                perform transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out;
                perform transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle; 
                perform transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out;
                perform transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out;
            }
            part vehicle_b_1:>vehicle_b = transportPassenger.vehicle{
                attribute :>> position3dVector = (0,0,0) [spatialCF];
                perform transportPassenger.b.providePower redefines providePower;
                perform transportPassenger.trigger;
            }
            connect driver.handPort to vehicle_b_1.ignitionCmdPort;
            connect road to vehicle_b_1.vehicleToRoadPort;
        }
    }
    package VehicleSuperSetModel{
        /* all of vehicleFamily is included in the superset model to enable subsetting a specific vehicle configuration*/
        package VariationPointDefinitions {
            variation part def TransmissionChoices:>Transmission {
                variant part transmissionAutomatic:TransmissionAutomatic;
                variant part transmissionManual:TransmissionManual;
            }
        }
        package VehiclePartsTree{
            public import VariationPointDefinitions::*;
            abstract part vehicleFamily {
                // variation with nested variation
                variation part engine:Engine{
                    variant part engine4Cyl:Engine4Cyl;
                    variant part engine6Cyl:Engine6Cyl{
                        part cylinder:Cylinder [6]{
                            variation attribute diameter:LengthValue{
                                variant attribute smallDiameter:LengthValue;
                                variant attribute largeDiagmeter:LengthValue;
                            }
                        }
                    }
                }
                // variation point based on variation of part definition
                part transmissionChoices:TransmissionChoices;
                // optional variation point
                part sunroof:Sunroof[0..1];
                // selection constraint
                assert constraint selectionConstraint{
                    (engine==engine::engine4Cyl and transmissionChoices==TransmissionChoices::transmissionManual) xor
                    (engine==engine::engine6Cyl and transmissionChoices==TransmissionChoices::transmissionAutomatic)
                }
                part driveshaft;
                part frontAxleAssembly;
                part rearAxleAssembly;
            }
        }
    }
    package SafetyandSecurityGroups {
        public import VehicleConfigurations::VehicleConfiguration_b::PartsTree::*;
        package SafetyGroup {
            /* Parts that contribute to safety. */
            public import vehicle_b::**;
            filter @Safety;
        }
        package SecurityGroup {
            /* Parts that contribute to security. */
            public import vehicle_b::**;
            filter @Security;
        }
        package SafetyandSecurityGroup {
            /* Parts that contribute to safety OR security. */
            public import vehicle_b::**;
            filter @Safety or @Security;
        }
        package MandatorySafetyGroup {
            /* Parts that contribute to safety AND are mandatory. */
            public import vehicle_b::**;
            filter @Safety and Safety::isMandatory;
        }
    }
    package Views_Viewpoints{
       package ViewpointDefinitions{
            viewpoint def BehaviorViewpoint;
            viewpoint def SafetyViewpoint{
                frame concern vs:VehicleSafety;
            }
            part def SafetyEngineer;
            concern def VehicleSafety {
                doc /* identify system safety features */
                subject;
                stakeholder se:SafetyEngineer;
            }
        }
        package ViewDefinitions{
            //public import Views to access rendering method library 
            public import Views::*;
            view def TreeView{
                render asTreeDiagram;
            }
            view def NestedView; 
            view def RelationshipView;
            view def TableView;
            view def PartsTreeView:>TreeView {
                filter @SysML::PartUsage;
            }
            view def PartsInterconnection:>NestedView;
        }
        package VehicleViews{
            public import ViewpointDefinitions::*;
            public import ViewDefinitions::*;
            public import VehicleConfigurations::VehicleConfiguration_b::*;
            view vehiclePartsTree_Safety:PartsTreeView{
                satisfy requirement sv:SafetyViewpoint;
                expose PartsTree::**;
                filter @Safety;
            }
        }
    }
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwExhibit,KwState,Ident,KwParallel,OpenCurly,
KwRef,Ident,Colon,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,KwAction,Ident,Semicolon,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,Ident,Semicolon,
KwDo,Ident,Semicolon,
KwExit,Ident,Semicolon,
KwConstraint,OpenCurly,Ident,LtEq,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
KwTransition,Ident,KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,Colon,Ident,KwVia,Ident,
KwIf,Ident,Dot,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,
KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
KwEntry,KwAction,Ident,Semicolon,
KwDo,Ident,OpenCurly,
KwOut,Ident,Semicolon,
CloseCurly,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,Ident,
KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,KwWhen,Ident,Dot,Ident,CloseAngle,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,
KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
KwExhibit,KwState,Ident,OpenCurly,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwDo,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwExhibit,KwState,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,OpenCurly,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwExhibit,KwState,Ident,KwParallel,OpenCurly,
KwState,Ident,OpenCurly,
KwEntry,KwAction,Ident,Semicolon,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,KwThen,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwExhibit,KwState,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwRef,KwItem,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAssert,KwConstraint,Ident,OpenCurly,Ident,Dot,Ident,LtEq,Ident,CloseCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPort,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwOut,KwItem,Ident,OpenSquare,Star,CloseSquare,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,OpenCurly,
CloseCurly,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Tilde,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Tilde,Ident,Semicolon,
KwFlow,KwOf,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwConstraint,OpenCurly,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAllocation,KwDef,Ident,OpenCurly,
KwEnd,Hash,Ident,Ident,Semicolon,
KwEnd,Hash,Ident,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwState,KwDef,Ident,Semicolon,
KwState,KwDef,Ident,Semicolon,
KwState,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,LtEq,Ident,CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,GtEq,Ident,CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,GtEq,Ident,CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,Ident,Semicolon,Ident,Semicolon,Ident,Semicolon,CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwEnum,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwEnum,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwEnum,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,Ident,Semicolon,Ident,Semicolon,CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,Ident,Semicolon,Ident,Semicolon,CloseCurly,
Ident,ColonGt,Ident,Eq,Ident,Slash,Ident,Semicolon,
Ident,ColonGt,Ident,Eq,Ident,Slash,Ident,Semicolon,
Ident,ColonGt,Ident,Eq,Ident,Slash,Ident,Semicolon,
Ident,ColonGt,Ident,Eq,Ident,Slash,Ident,Semicolon,
LineComment,
Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
Ident,Colon,Ident,Eq,DecimalValue,Slash,Ident,ColonColon,Ident,Semicolon,
Ident,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwMetadata,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwState,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
LineComment,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwOccurrence,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwOccurrence,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Slash,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
Hash,Ident,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,StarStar,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwAllocation,Ident,Colon,Ident,
KwAllocate,Ident,KwTo,Ident,OpenCurly,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwAllocate,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwAllocate,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,Ident,Plus,Ident,Plus,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAttribute,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwRef,KwItem,KwRedefines,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
LineComment,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
Hash,Ident,KwAttribute,Ident,KwRedefines,Ident,Eq,Ident,Plus,Ident,Plus,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAttribute,KwRedefines,Ident,KwDefault,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Eq,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwPort,Ident,Colon,Ident,KwRedefines,Ident,OpenCurly,
KwIn,KwItem,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,KwRedefines,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPerform,Ident,ColonColon,Ident,KwRedefines,Ident,Semicolon,
KwPerform,Ident,ColonColon,Ident,KwRedefines,Ident,Semicolon,
KwPerform,Ident,ColonColon,Ident,KwRedefines,Ident,Semicolon,
KwPerform,Ident,ColonColon,Ident,KwRedefines,Ident,Semicolon,
KwExhibit,KwState,Ident,KwRedefines,Ident,Semicolon,
LineComment,
KwItem,ColonGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
Ident,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
Ident,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
Ident,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwRef,KwItem,KwRedefines,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,
KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,
KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,KwRedefines,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,
KwAction,Ident,Semicolon,
CloseCurly,
KwSatisfy,Ident,ColonColon,Ident,KwBy,Ident,Dot,Ident,OpenCurly,
KwRequirement,Ident,ColonGtGt,Ident,OpenCurly,
KwSubject,Ident,KwRedefines,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,KwRedefines,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwExhibit,KwState,Ident,KwRedefines,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwPart,Ident,OpenCurly,At,Ident,Semicolon,CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,At,Ident,Semicolon,CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
CloseCurly,
LineComment,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAllocate,Ident,ColonColon,Ident,Dot,Ident,KwTo,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwSatisfy,Ident,ColonColon,Ident,KwBy,Ident,OpenCurly,
KwRequirement,Ident,ColonGtGt,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwOut,Ident,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
LineComment,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
LineComment,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPerform,KwAction,Ident,OpenCurly,
KwAction,Ident,KwSend,Ident,KwVia,Ident,Dot,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,Ident,KwAccept,Ident,Colon,Ident,KwVia,Ident,Dot,Ident,Semicolon,
KwFlow,KwOf,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,KwOf,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,KwSend,Ident,KwVia,Ident,Dot,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,Ident,KwAccept,Ident,Colon,Ident,KwVia,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwFirst,Ident,Dot,Ident,KwThen,Ident,Dot,Ident,Semicolon,
KwMessage,KwOf,Ident,Colon,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,KwOf,Ident,Colon,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwOccurrence,Ident,OpenCurly,
KwPart,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
LineComment,
KwEvent,KwOccurrence,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwOccurrence,Ident,OpenCurly,
KwPart,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
LineComment,
KwEvent,KwOccurrence,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwMessage,Ident,KwOf,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwItem,Ident,Semicolon,
KwDependency,KwFrom,Ident,KwTo,Ident,Semicolon,
KwRequirement,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,KwRedefines,Ident,KwDefault,Ident,Dot,Ident,Plus,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,Ident,EqEq,Ident,CloseCurly,
CloseCurly,
KwAllocate,Ident,KwTo,Ident,ColonColon,Ident,Dot,Ident,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,Ident,LtEq,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,Ident,LtEq,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
LineComment,
LineComment,
At,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,StringValue,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwRequirement,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,KwDefault,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
LineComment,
Hash,Ident,KwConnection,OpenCurly,
KwEnd,Hash,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
Hash,Ident,KwDependency,Ident,KwTo,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
LineComment,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,Colon,Ident,
KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Semicolon,
CloseCurly,
LineComment,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,Colon,Ident,
KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColonGt,Ident,Dot,Ident,OpenCurly,
KwInterface,Ident,ColonGt,Ident,
KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwPort,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,Colon,Ident,
KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColonGt,Ident,Dot,Ident,OpenCurly,
KwInterface,Ident,ColonGt,Ident,
KwConnect,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,Ident,ColonColonGt,Ident,Dot,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Star,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Star,Ident,CloseSquare,Semicolon,
CloseCurly,
KwInterface,Ident,ColonGt,Ident,
KwConnect,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,Ident,ColonColonGt,Ident,Dot,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Star,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Star,Ident,CloseSquare,Semicolon,
CloseCurly,
KwInterface,Ident,ColonGt,Ident,
KwConnect,Ident,ColonColonGt,Ident,Dot,Ident,KwTo,Ident,ColonColonGt,Ident,Dot,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Star,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Star,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,StarStar,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwAttribute,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
KwReturn,Ident,ColonGt,Ident,Eq,DecimalValue,Slash,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Star,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwConstraint,OpenCurly,Ident,EqEq,DecimalValue,Slash,DecimalValue,Star,Ident,Star,Ident,StarStar,OpenParen,Minus,DecimalValue,CloseParen,Slash,Ident,CloseCurly,
KwReturn,Ident,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAnalysis,Ident,OpenCurly,
KwSubject,Eq,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
KwDoc,RegularComment,
KwRequire,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
LineComment,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
KwReturn,KwAttribute,Ident,ColonGt,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
RegularComment,
At,Ident,KwAbout,Ident,ColonColon,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
Ident,Eq,StringValue,Semicolon,
CloseCurly,
LineComment,
At,Ident,KwAbout,Ident,ColonColon,Ident,OpenCurly,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
At,Ident,KwAbout,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,OpenCurly,
Ident,OpenCurly,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,KwRedefines,Ident,OpenCurly,
KwPart,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,Dot,DecimalValue,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,KwRedefines,Ident,OpenCurly,
KwPart,Ident,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,Dot,DecimalValue,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwObjective,Colon,Ident,Semicolon,
RegularComment,
KwCalc,ColonGt,Ident,OpenCurly,
KwIn,KwPart,Ident,ColonGt,Ident,Semicolon,
KwReturn,KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,ColonGt,Ident,OpenCurly,
KwIn,KwPart,Ident,ColonGt,Ident,Semicolon,
KwReturn,KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwReturn,KwPart,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,StarStar,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwVerification,KwDef,Ident,Semicolon,
KwVerification,KwDef,Ident,Semicolon,
KwVerification,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwVerification,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,ColonGt,Ident,Semicolon,
KwActor,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwObjective,OpenCurly,
KwVerify,Ident,Dot,Ident,OpenCurly,
KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
At,Ident,OpenCurly,
Ident,Eq,OpenParen,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,CloseParen,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwOut,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwThen,KwAction,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwOut,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,OpenParen,Ident,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwReturn,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwIndividual,Ident,Colon,Ident,OpenCurly,
KwTimeslice,Ident,OpenCurly,
KwSnapshot,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,Dot,DecimalValue,Semicolon,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Slash,Ident,StarStar,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,Semicolon,
KwSnapshot,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwSnapshot,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,Dot,DecimalValue,Semicolon,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,Eq,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Slash,Ident,StarStar,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,Semicolon,
KwSnapshot,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwSnapshot,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,Dot,DecimalValue,Semicolon,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Slash,Ident,StarStar,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,Semicolon,
KwSnapshot,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,StarStar,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,OpenCurly,
CloseCurly,
KwExhibit,KwState,Ident,OpenCurly,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,KwThen,Ident,Semicolon,
LineComment,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,Ident,Eq,Ident,ColonColon,Ident,CloseParen,KwVia,Ident,
KwThen,Ident,Semicolon,
LineComment,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,Ident,Eq,Ident,ColonColon,Ident,CloseParen,KwVia,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwRequirement,Ident,Semicolon,
KwUse,KwCase,KwDef,Ident,OpenCurly,
KwObjective,Ident,OpenCurly,
KwDoc,RegularComment,
KwRequire,Ident,Semicolon,
CloseCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Semicolon,
KwActor,Ident,Semicolon,
KwActor,Ident,Semicolon,
KwActor,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwInclude,KwUse,KwCase,Ident,ColonGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwInclude,KwUse,KwCase,Ident,ColonGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwUse,KwCase,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwThen,KwAction,Ident,Semicolon,
KwThen,KwAction,Ident,Semicolon,
KwThen,KwAction,Ident,Semicolon,
CloseCurly,
KwUse,KwCase,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwActor,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,BangEq,KwNull,KwXor,Ident,BangEq,KwNull,CloseCurly,
CloseCurly,
KwUse,KwCase,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,Semicolon,
KwThen,KwAction,Ident,Semicolon,
KwThen,KwAction,Ident,Semicolon,
KwThen,KwAction,Ident,Semicolon,
CloseCurly,
KwUse,KwCase,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwActor,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,BangEq,KwNull,KwXor,Ident,BangEq,KwNull,CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwUse,KwCase,Ident,Colon,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwThen,KwAction,Ident,KwAccept,Ident,Colon,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
CloseCurly,
KwThen,KwAction,Ident,OpenCurly,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
LineComment,
KwUse,KwCase,Ident,Colon,Ident,OpenCurly,
LineComment,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,KwSubsets,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwJoin,Ident,Semicolon,
KwJoin,Ident,Semicolon,
KwJoin,Ident,Semicolon,
KwAction,Ident,KwAccept,Ident,Colon,Ident,Semicolon,
LineComment,
KwFirst,Ident,Semicolon,
KwThen,KwFork,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
LineComment,
KwFork,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFork,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,
Hash,Ident,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPerform,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,
KwPerform,Ident,Dot,Ident,Dot,Ident,KwRedefines,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
RegularComment,
KwPackage,Ident,OpenCurly,
KwVariation,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,Semicolon,
KwVariant,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwAbstract,KwPart,Ident,OpenCurly,
LineComment,
KwVariation,KwPart,Ident,Colon,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,Semicolon,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwVariation,KwAttribute,Ident,Colon,Ident,OpenCurly,
KwVariant,KwAttribute,Ident,Colon,Ident,Semicolon,
KwVariant,KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
LineComment,
KwPart,Ident,Colon,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwAssert,KwConstraint,Ident,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,Ident,EqEq,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,Ident,EqEq,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,KwOr,At,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,KwAnd,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwViewpoint,KwDef,Ident,Semicolon,
KwViewpoint,KwDef,Ident,OpenCurly,
KwFrame,KwConcern,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConcern,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Semicolon,
KwStakeholder,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwView,KwDef,Ident,OpenCurly,
KwRender,Ident,Semicolon,
CloseCurly,
KwView,KwDef,Ident,Semicolon,
KwView,KwDef,Ident,Semicolon,
KwView,KwDef,Ident,Semicolon,
KwView,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwFilter,At,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwView,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwView,Ident,Colon,Ident,OpenCurly,
KwSatisfy,KwRequirement,Ident,Colon,Ident,Semicolon,
KwExpose,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'SimpleVehicleModel'
    (line_comment)
    (import_decl public 'Definitions::*')
    (import_decl public 'ISQ::*')
    (package_def 'Definitions'
      (import_decl public 'PartDefinitions::*')
      (import_decl public 'PortDefinitions::*')
      (import_decl public 'ItemDefinitions::*')
      (import_decl public 'SignalDefinitions::*')
      (import_decl public 'InterfaceDefinitions::*')
      (import_decl public 'AllocationDefinitions::*')
      (import_decl public 'ActionDefinitions::*')
      (import_decl public 'StateDefinitions::*')
      (import_decl public 'RequirementDefinitions::*')
      (import_decl public 'AttributeDefinitions::*')
      (import_decl public 'IndividualDefinitions::*')
      (import_decl public 'MetadataDefinitions::**')
      (import_decl public 'KeyWord_MetadataDefinitions::*')
      (package_def 'PartDefinitions'
        (part_def 'Vehicle'
          (attribute_usage 'mass' :> 'ISQ::mass')
          (attribute_usage 'dryMass' :> 'ISQ::mass')
          (attribute_usage 'cargoMass' :> 'ISQ::mass')
          (attribute_usage 'position' :> 'ISQ::length')
          (attribute_usage 'velocity' :> 'ISQ::speed')
          (attribute_usage 'acceleration' :> 'ISQ::acceleration')
          (attribute_usage 'electricalPower' :> 'ISQ::power')
          (attribute_usage 'Tmax' :> 'ISQ::temperature')
          (attribute_usage 'maintenanceTime' : 'Time::DateTime')
          (attribute_usage 'brakePedalDepressed' : 'Boolean')
          (port_usage 'ignitionCmdPort' : 'IgnitionCmdPort')
          (port_usage 'pwrCmdPort' : 'PwrCmdPort')
          (port_usage 'vehicleToRoadPort' : 'VehicleToRoadPort')
          (port_usage 'statusPort' : 'StatusPort')
          (perform_action 'providePower')
          (perform_action 'provideBraking')
          (perform_action 'controlDirection')
          (perform_action 'performSelfTest')
          (perform_action 'applyParkingBrake')
          (perform_action 'senseTemperature')
          (exhibit_state parallel 'vehicleStates'
            (ref_usage ref 'controller' : 'VehicleController')
            (state_usage 'operatingStates'
              (entry_action 'initial')
              (state_usage 'off')
              (state_usage 'starting')
              (state_usage 'on'
                (entry_action 'performSelfTest')
                (do_action 'providePower')
                (exit_action 'applyParkingBrake')
                (constraint_usage
                  (result_expr_member)))
              (transition_usage)
              (transition_usage 'off_To_starting')
              (transition_usage 'starting_To_on')
              (transition_usage 'on_To_off'))
            (state_usage 'healthStates'
              (entry_action 'initial')
              (do_action 'senseTemperature'
                (default_ref_usage out 'temp'))
              (state_usage 'normal')
              (state_usage 'maintenance')
              (state_usage 'degraded')
              (transition_usage)
              (transition_usage 'normal_To_maintenance')
              (transition_usage 'normal_To_degraded')
              (transition_usage 'maintenance_To_normal')
              (transition_usage 'degraded_To_normal'))))
        (part_def 'Engine'
          (attribute_usage 'mass' :> 'ISQ::mass')
          (attribute_usage 'peakHorsePower' :> 'ISQ::power')
          (attribute_usage 'fuelEfficiency' : 'Real')
          (attribute_usage 'cost' : 'Real')
          (attribute_usage 'displacement' :> 'ISQ::volume')
          (port_usage 'engineControlPort' : ~'ControlPort')
          (port_usage 'fuelInPort' : ~'FuelPort')
          (port_usage 'fuelCmdPort' : 'FuelCmdPort')
          (port_usage 'drivePwrPort' : 'DrivePwrPort')
          (port_usage 'ignitionCmdPort' : 'IgnitionCmdPort')
          (port_usage 'flyWheelPort')
          (perform_action 'generateTorque')
          (exhibit_state 'engineStates'
            (state_usage 'off')
            (state_usage 'starting')
            (state_usage 'on'
              (do_action 'generateTorque'))))
        (part_def 'StarterMotor'
          (port_usage 'gearPort' : 'GearPort'))
        (part_def 'Cylinder')
        (part_def 'Transmission'
          (attribute_usage 'gearRatio' : 'Real')
          (port_usage 'clutchPort' : ~'DrivePwrPort')
          (exhibit_state 'transmissionStates'))
        (part_def 'Driveshaft')
        (part_def 'AxleAssembly')
        (part_def 'Axle'
          (attribute_usage 'mass' :> 'ISQ::mass'))
        (part_def 'FrontAxle' :> 'Axle'
          (attribute_usage 'steeringAngle' :> 'ISQ::angularMeasure'))
        (part_def 'HalfAxle'
          (port_usage 'shankCompositePort' : 'ShankCompositePort'))
        (part_def 'Differential')
        (part_def 'Wheel'
          (attribute_usage 'diameter' : 'LengthValue')
          (port_usage 'lugNutCompositePort' : 'LugNutCompositePort'))
        (part_def 'Hub'
          (port_usage 'shankCompositePort' : 'ShankCompositePort'))
        (part_def abstract 'Software')
        (part_def 'VehicleSoftware' :> 'Software')
        (part_def 'VehicleController' :> 'Software'
          (port_usage 'controlPort' : 'ControlPort')
          (exhibit_state parallel 'controllerStates'
            (state_usage 'operatingStates'
              (entry_action 'initial')
              (state_usage 'off')
              (state_usage 'on')
              (transition_usage)
              (transition_usage)
              (transition_usage))))
        (part_def 'CruiseController' :> 'Software'
          (port_usage 'setSpeedPort' : ~'SetSpeedPort')
          (port_usage 'speedSensorPort' : ~'SpeedSensorPort')
          (port_usage 'cruiseControlPort' : 'CruiseControlPort')
          (exhibit_state 'cruiseControllerStates'))
        (part_def 'SpeedSensor'
          (port_usage 'speedSensorPort' : 'SpeedSensorPort'))
        (part_def 'FuelTank'
          (attribute_usage 'mass' :> 'ISQ::mass')
          (item_usage ref 'fuel' : 'Fuel'
            (attribute_usage :>> 'fuelMass'))
          (attribute_usage 'fuelKind' : 'FuelKind')
          (attribute_usage 'fuelMassMax' :> 'ISQ::mass')
          (sysml_decl 'fuelConstraint'
            (result_expr_member))
          (port_usage 'fuelOutPort' : 'FuelPort')
          (port_usage 'fuelInPort' : ~'FuelPort'))
        (part_def 'BodyAssy')
        (part_def 'Body'
          (attribute_usage 'color' : 'Colors'))
        (part_def 'Thermostat')
        (part_def 'WaterHose')
        (part_def 'Road'
          (attribute_usage 'incline' : 'Real')
          (attribute_usage 'friction' : 'Real'))
        (part_def 'Engine4Cyl')
        (part_def 'Engine6Cyl')
        (part_def 'TransmissionChoices')
        (part_def 'TransmissionAutomatic')
        (part_def 'TransmissionManual')
        (part_def 'Sunroof')
        (line_comment)
        (part_def 'ElectricalGenerator')
        (part_def 'TorqueGenerator')
        (part_def 'SteeringSubsystem')
        (part_def 'BrakingSubsystem'))
      (package_def 'PortDefinitions'
        (port_def 'IgnitionCmdPort'
          (item_usage in 'ignitionCmd' : 'IgnitionCmd'))
        (port_def 'StatusPort')
        (port_def 'GearPort')
        (port_def 'PwrCmdPort'
          (item_usage in 'pwrCmd' : 'PwrCmd'))
        (port_def 'FuelCmdPort' :> 'PwrCmdPort'
          (item_usage in 'fuelCmd' : 'FuelCmd' :>> 'pwrCmd'))
        (port_def 'FuelPort'
          (item_usage out 'fuel' : 'Fuel'))
        (port_def 'DrivePwrPort'
          (default_ref_usage out 'torque' : 'Torque'))
        (port_def 'ShaftPort_a')
        (port_def 'ShaftPort_b')
        (port_def 'ShaftPort_c')
        (port_def 'ShaftPort_d')
        (port_def 'DiffPort')
        (port_def 'AxlePort')
        (port_def 'AxleToWheelPort')
        (port_def 'WheelToAxlePort')
        (port_def 'WheelToRoadPort')
        (port_def 'LugNutCompositePort'
          (port_usage 'lugNutPort' : 'LugNutPort' multiplicity))
        (port_def 'ShankCompositePort'
          (port_usage 'shankPort' : 'ShankPort' multiplicity))
        (port_def 'LugNutPort'
          (attribute_usage 'threadDia')
          (attribute_usage 'threadPitch'))
        (port_def 'ShankPort'
          (attribute_usage 'threadDia')
          (attribute_usage 'threadPitch')
          (attribute_usage 'shaftLength'))
        (port_def 'VehicleToRoadPort')
        (port_def 'ControlPort')
        (port_def 'CruiseControlPort' :> 'ControlPort')
        (port_def 'SpeedSensorPort')
        (port_def 'SetSpeedPort')
        (port_def 'DriverCmdPort'
          (item_usage out 'driverCmd' : 'DriverCmd' multiplicity))
        (port_def 'HandPort' :> 'DriverCmdPort'
          (item_usage out 'ignitionCmd' : 'IgnitionCmd' :> 'driverCmd')
          (item_usage out 'pwrCmd' : 'PwrCmd' :> 'driverCmd')))
      (package_def 'ItemDefinitions'
        (item_def 'PwrCmd'
          (attribute_usage 'throttleLevel' : 'Real'))
        (item_def 'FuelCmd' :> 'PwrCmd')
        (item_def 'Fuel'
          (attribute_usage 'fuelMass' :> 'ISQ::mass'))
        (item_def 'SensedSpeed'
          (attribute_usage 'speed' :> 'ISQ::speed')))
      (package_def 'SignalDefinitions'
        (item_def 'Cmd')
        (item_def 'DriverCmd')
        (item_def 'IgnitionCmd' :> 'DriverCmd'
          (attribute_usage 'ignitionOnOff' : 'IgnitionOnOff'))
        (item_def 'EngineStatus')
        (attribute_def 'VehicleStartSignal')
        (attribute_def 'VehicleOnSignal')
        (attribute_def 'VehicleOffSignal')
        (attribute_def 'StartSignal')
        (attribute_def 'OffSignal')
        (attribute_def 'OverTemp')
        (attribute_def 'ReturnToNormal')
        (attribute_def 'SetSpeed' :> 'Real'))
      (package_def 'InterfaceDefinitions'
        (interface_def 'EngineToTransmissionInterface'
          (interface_end end 'p1' : 'DrivePwrPort')
          (interface_end end 'p2' : 'DrivePwrPort')
          (flow_usage 'p1'))
        (interface_def 'FuelInterface'
          (interface_end end 'fuelOutPort' : 'FuelPort')
          (interface_end end 'fuelInPort' : 'FuelPort')
          (flow_usage 'of'))
        (interface_def 'WheelFastenerInterface'
          (interface_end end 'lugNutPort' : 'LugNutPort')
          (interface_end end 'shankPort' : 'ShankPort')
          (attribute_usage 'maxTorque' : 'Torque')
          (constraint_usage
            (result_expr_member)))
        (interface_def 'WheelHubInterface'
          (interface_end end 'lugNutCompositePort' : 'LugNutCompositePort')
          (interface_end end 'shankCompositePort' : 'ShankCompositePort')
          (interface_usage 'WheelFastenerInterface' 'wheelFastenerInterface' multiplicity
            (connector_end)
            (connector_end))))
      (package_def 'AllocationDefinitions'
        (allocation_def 'LogicalToPhysical'
          (interface_end end #'logical' 'logicalEnd')
          (interface_end end #'physical' 'physicalEnd')))
      (package_def 'ActionDefinitions'
        (action_def 'ProvidePower'
          (item_usage in 'pwrCmd' : 'PwrCmd')
          (default_ref_usage out 'wheelToRoadTorque' : 'Torque' multiplicity))
        (action_def 'GenerateTorque'
          (item_usage in 'fuelCmd' : 'FuelCmd')
          (default_ref_usage out 'engineTorque' : 'Torque'))
        (action_def 'AmplifyTorque'
          (default_ref_usage in 'engineTorque' : 'Torque')
          (default_ref_usage out 'transmissionTorque' : 'Torque'))
        (action_def 'TransferTorque'
          (default_ref_usage in 'transmissionTorque' : 'Torque')
          (default_ref_usage out 'driveshaftTorque' : 'Torque'))
        (action_def 'DistributeTorque'
          (default_ref_usage in 'driveshaftTorque' : 'Torque')
          (default_ref_usage out 'wheelToRoadTorque' : 'Torque' multiplicity))
        (action_def 'PerformSelfTest')
        (action_def 'ApplyParkingBrake')
        (action_def 'SenseTemperature'
          (default_ref_usage out 'temp' : 'ISQ::TemperatureValue')))
      (package_def 'StateDefinitions'
        (state_def 'VehicleStates')
        (state_def 'ControllerStates')
        (state_def 'CruiseControllerStates'))
      (package_def 'RequirementDefinitions'
        (requirement_def 'MassRequirement'
          (documentation)
          (attribute_usage 'massRequired' :> 'ISQ::mass')
          (attribute_usage 'massActual' :> 'ISQ::mass')
          (sysml_decl
            (result_expr_member)))
        (requirement_def 'ReliabilityRequirement'
          (documentation)
          (attribute_usage 'reliabilityRequired' : 'Real')
          (attribute_usage 'reliabilityActual' : 'Real')
          (sysml_decl
            (result_expr_member)))
        (requirement_def 'TorqueGenerationRequirement'
          (documentation)
          (sysml_decl 'generateTorque' : 'ActionDefinitions::GenerateTorque'))
        (requirement_def 'DrivePowerOutputRequirement'
          (documentation))
        (requirement_def 'FuelEconomyRequirement'
          (documentation)
          (attribute_usage 'actualFuelEconomy' :> 'distancePerVolume')
          (attribute_usage 'requiredFuelEconomy' :> 'distancePerVolume')
          (sysml_decl
            (result_expr_member))))
      (package_def 'AttributeDefinitions'
        (import_decl public 'ScalarValues::*')
        (import_decl public 'Quantities::*')
        (import_decl public 'MeasurementReferences::DerivedUnit')
        (import_decl public 'SIPrefixes::kilo')
        (line_comment)
        (import_decl public 'NumericalFunctions::*')
        (import_decl public 'SI::*')
        (import_decl public 'USCustomaryUnits::*')
        (alias_member 'Torque' for 'ISQ::TorqueValue')
        (enum_def 'Colors'
          (enum_value 'black')
          (enum_value 'grey')
          (enum_value 'red'))
        (enum_def 'DiameterChoices' :> 'ISQ::LengthValue'
          (malformed)
          (malformed)
          (malformed))
        (attribute_usage 'cylinderDiameter' : 'DiameterChoices' value)
        (enum_def 'IgnitionOnOff'
          (enum_value 'on')
          (enum_value 'off'))
        (enum_def 'FuelKind'
          (enum_value 'gas')
          (enum_value 'diesel'))
        (feature_def 'distancePerVolume' :> 'scalarQuantities' value)
        (feature_def 'timePerDistance' :> 'scalarQuantities' value)
        (feature_def 'volumePerDistance' :> 'scalarQuantities' value)
        (feature_def 'volumePerTime' :> 'scalarQuantities' value)
        (line_comment)
        (feature_def 'kpl' : 'DerivedUnit' value)
        (feature_def 'rpm' : 'DerivedUnit' value)
        (feature_def 'kW' : 'DerivedUnit' value))
      (package_def 'IndividualDefinitions'
        (individual_def individual 'VehicleRoadContext_1' :> 'GenericContext::Context')
        (individual_def individual 'Vehicle_1' :> 'Vehicle')
        (individual_def individual 'FrontAxleAssembly_1' :> 'AxleAssembly')
        (individual_def individual 'FrontAxle_1' :> 'FrontAxle')
        (individual_def individual 'Wheel_1' :> 'Wheel')
        (individual_def individual 'Wheel_2' :> 'Wheel')
        (individual_def individual 'RearAxleAssembly_1' :> 'AxleAssembly')
        (individual_def individual 'Road_1' :> 'Road'))
      (package_def 'MetadataDefinitions'
        (import_decl public 'AnalysisTooling::*')
        (metadata_def 'Safety'
          (attribute_usage 'isMandatory' : 'Boolean'))
        (metadata_def 'Security'))
      (package_def 'KeyWord_MetadataDefinitions'
        (import_decl public 'Metaobjects::SemanticMetadata')
        (line_comment)
        (state_usage 'failureModes' multiplicity nonunique)
        (line_comment)
        (metadata_def 'failureMode' :> 'SemanticMetadata'
          (default_ref_usage :>> 'baseType' value))
        (occurrence_usage 'logicalOccurrences' multiplicity nonunique)
        (metadata_def 'logical' :> 'SemanticMetadata'
          (default_ref_usage :>> 'baseType' value))
        (occurrence_usage 'physicalOccurrences' multiplicity nonunique)
        (metadata_def 'physical' :> 'SemanticMetadata'
          (default_ref_usage :>> 'baseType' value)))
      (package_def 'GenericContext'
        (part_def 'Context'
          (attribute_usage 'time' : 'TimeValue')
          (attribute_usage 'spatialCF' : 'CartesianSpatial3dCoordinateFrame' multiplicity
            (default_ref_usage :>> 'mRefs' value))
          (attribute_usage 'velocityCF' : 'CartesianVelocity3dCoordinateFrame' multiplicity value)
          (attribute_usage 'accelarationCF' : 'CartesianAcceleration3dCoordinateFrame' multiplicity value))))
    (package_def 'VehicleLogicalConfiguration'
      (package_def 'PartsTree'
        (part_usage #'logical' 'vehicleLogical' : 'Vehicle'
          (part_usage 'torqueGenerator' : 'TorqueGenerator'
            (action_usage 'generateTorque'))
          (part_usage 'electricalGenerator' : 'ElectricalGenerator'
            (action_usage 'generateElectricity'))
          (part_usage 'steeringSystem' : 'SteeringSubsystem')
          (part_usage 'brakingSubsystem' : 'BrakingSubsystem'))))
    (package_def 'VehicleLogicalToPhysicalAllocation'
      (import_decl public 'VehicleConfigurations::VehicleConfiguration_b::PartsTree::**')
      (import_decl public 'VehicleLogicalConfiguration::PartsTree::*')
      (allocation_usage 'LogicalToPhysical' 'vehicleLogicalToPhysicalAllocation'
        (connector_end)
        (connector_end)
        (allocation_usage
          (connector_end)
          (connector_end)
          (allocation_usage
            (connector_end)
            (connector_end)))
        (allocation_usage
          (connector_end)
          (connector_end)
          (allocation_usage
            (connector_end)
            (connector_end)))))
    (package_def 'VehicleConfigurations'
      (package_def 'VehicleConfiguration_a'
        (package_def 'PartsTree'
          (part_usage 'vehicle_a' : 'Vehicle'
            (attribute_usage 'mass' :>> 'Vehicle::mass' value)
            (attribute_usage 'dryMass' :>> 'Vehicle::dryMass' value)
            (attribute_usage :>> 'Vehicle::cargoMass' value)
            (attribute_usage 'partMasses' :> 'ISQ::mass' multiplicity nonunique)
            (part_usage 'fuelTank' : 'FuelTank'
              (attribute_usage :>> 'mass' value)
              (item_usage ref :>> 'fuel'
                (attribute_usage :>> 'fuelMass' value)))
            (part_usage 'frontAxleAssembly' : 'AxleAssembly'
              (attribute_usage 'mass' :> 'ISQ::mass' value)
              (part_usage 'frontAxle' : 'Axle')
              (part_usage 'frontWheels' : 'Wheel' multiplicity))
            (part_usage 'rearAxleAssembly' : 'AxleAssembly'
              (attribute_usage 'mass' :> 'ISQ::mass' value)
              (attribute_usage 'driveTrainEfficiency' : 'Real' value)
              (part_usage 'rearAxle' : 'Axle')
              (part_usage 'rearWheels' : 'Wheel' multiplicity
                (attribute_usage :>> 'diameter')))))
        (package_def 'ActionTree')
        (package_def 'Requirements'))
      (package_def 'VehicleConfiguration_b'
        (line_comment)
        (import_decl public 'ShapeItems::Box')
        (import_decl public 'ParametersOfInterestMetadata::mop')
        (import_decl public 'ModelingMetadata::*')
        (line_comment)
        (package_def 'PartsTree'
          (part_usage 'vehicle_b' : 'Vehicle'
            (attribute_usage #'mop' 'mass' :>> 'mass' value)
            (attribute_usage 'dryMass' :>> 'dryMass' value)
            (attribute_usage :>> 'cargoMass' value)
            (attribute_usage 'partMasses' value)
            (attribute_usage 'avgFuelEconomy' :> 'distancePerVolume')
            (port_usage 'fuelCmdPort' : 'FuelCmdPort' :>> 'pwrCmdPort'
              (item_usage in 'fuelCmd' :>> 'pwrCmd'))
            (port_usage 'setSpeedPort' : ~'SetSpeedPort')
            (port_usage 'vehicleToRoadPort' :>> 'vehicleToRoadPort'
              (port_usage 'wheelToRoadPort1' : 'WheelToRoadPort')
              (port_usage 'wheelToRoadPort2' : 'WheelToRoadPort'))
            (perform_action :>> 'ActionTree::providePower')
            (default_ref_usage :>> 'providePower')
            (perform_action :>> 'ActionTree::performSelfTest')
            (default_ref_usage :>> 'performSelfTest')
            (perform_action :>> 'ActionTree::applyParkingBrake')
            (default_ref_usage :>> 'applyParkingBrake')
            (perform_action :>> 'ActionTree::senseTemperature')
            (default_ref_usage :>> 'senseTemperature')
            (exhibit_state 'vehicleStates' :>> 'vehicleStates')
            (line_comment)
            (item_usage :> 'envelopingShapes' : 'Box' multiplicity
              (default_ref_usage 'length1' :>> 'length' value)
              (default_ref_usage 'width1' :>> 'width' value)
              (default_ref_usage 'height1' :>> 'height' value))
            (part_usage 'fuelTank' : 'FuelTank'
              (attribute_usage :>> 'mass' value)
              (item_usage ref :>> 'fuel'
                (attribute_usage :>> 'fuelMass' value))
              (attribute_usage :>> 'fuelMassMax' value))
            (part_usage 'frontAxleAssembly' : 'AxleAssembly'
              (attribute_usage 'mass' :> 'ISQ::mass' value)
              (port_usage 'shaftPort_d' : 'ShaftPort_d')
              (part_usage 'frontAxle' : 'FrontAxle')
              (part_usage 'frontWheels' : 'Wheel' multiplicity))
            (part_usage 'rearAxleAssembly' : 'AxleAssembly'
              (attribute_usage 'mass' :> 'ISQ::mass' value)
              (attribute_usage 'driveTrainEfficiency' : 'Real' value)
              (port_usage 'shaftPort_d' : 'ShaftPort_d')
              (perform_action :>> 'providePower.distributeTorque')
              (part_usage 'rearWheel1' : 'Wheel'
                (attribute_usage :>> 'diameter')
                (port_usage 'wheelToRoadPort' : 'WheelToRoadPort')
                (port_usage 'lugNutCompositePort' :>> 'lugNutCompositePort'
                  (port_usage 'lugNutPort' :>> 'lugNutPort' multiplicity)))
              (part_usage 'rearWheel2' : 'Wheel'
                (attribute_usage :>> 'diameter')
                (port_usage 'wheelToRoadPort' : 'WheelToRoadPort')
                (port_usage 'lugNutCompositePort' :>> 'lugNutCompositePort'
                  (port_usage 'lugNutPort' :>> 'lugNutPort' multiplicity)))
              (part_usage 'differential' : 'Differential'
                (port_usage 'shaftPort_d' : 'ShaftPort_d')
                (port_usage 'leftDiffPort' : 'DiffPort')
                (port_usage 'rightDiffPort' : 'DiffPort'))
              (part_usage 'rearAxle'
                (part_usage 'leftHalfAxle' : 'HalfAxle'
                  (port_usage 'leftAxleToDiffPort' : 'AxlePort')
                  (port_usage 'shankCompositePort' :>> 'shankCompositePort'
                    (port_usage 'shankPort' :>> 'shankPort' multiplicity)))
                (part_usage 'rightHalfAxle' : 'HalfAxle'
                  (port_usage 'rightAxleToDiffPort' : 'AxlePort')
                  (port_usage 'shankCompositePort' :>> 'shankCompositePort'
                    (port_usage 'shankPort' :>> 'shankPort' multiplicity))))
              (binding_as_usage
                (connector_end)
                (connector_end))
              (connection_usage
                (connector_end)
                (connector_end))
              (connection_usage
                (connector_end)
                (connector_end))
              (interface_usage 'WheelHubInterface' 'wheelToleftHalAxleInterface'
                (connector_end)
                (connector_end))
              (interface_usage 'WheelHubInterface' 'wheelTorightHalAxleInterface'
                (connector_end)
                (connector_end)))
            (part_usage 'starterMotor' : 'StarterMotor')
            (part_usage 'engine' : 'Engine'
              (perform_action :>> 'providePower.generateTorque')
              (default_ref_usage :>> 'generateTorque')
              (part_usage 'cylinders' : 'Cylinder' multiplicity)
              (part_usage 'alternator'
                (action_usage 'generateElectricity'))
              (malformed))
            (part_usage 'transmission' : 'Transmission'
              (attribute_usage 'mass' :> 'ISQ::mass' value)
              (port_usage 'shaftPort_a' : 'ShaftPort_a')
              (perform_action :>> 'providePower.amplifyTorque'))
            (part_usage 'driveshaft' : 'Driveshaft'
              (attribute_usage 'mass' :> 'ISQ::mass' value)
              (port_usage 'shaftPort_b' : 'ShaftPort_b')
              (port_usage 'shaftPort_c' : 'ShaftPort_c')
              (perform_action :>> 'providePower.transferTorque'))
            (part_usage 'vehicleSoftware' : 'VehicleSoftware'
              (part_usage 'vehicleController' : 'VehicleController'
                (exhibit_state 'controllerStates' :>> 'controllerStates')
                (part_usage 'cruiseController' : 'CruiseController')))
            (part_usage 'speedSensor' : 'SpeedSensor')
            (line_comment)
            (part_usage 'bodyAssy' : 'BodyAssy'
              (part_usage 'body' : 'Body'
                (attribute_usage :>> 'color' value))
              (part_usage 'bumper'
                (metadata_feature typed 'Safety'
                  (feature_def 'isMandatory' value)))
              (part_usage 'keylessEntry'
                (metadata_feature typed 'Security')))
            (part_usage 'interior'
              (part_usage 'alarm'
                (metadata_feature typed 'Security'))
              (part_usage 'seatBelt' multiplicity
                (metadata_feature typed 'Safety'
                  (feature_def 'isMandatory' value)))
              (part_usage 'frontSeat' multiplicity)
              (part_usage 'driverAirBag'
                (metadata_feature typed 'Safety'
                  (feature_def 'isMandatory' value))))
            (line_comment)
            (binding_as_usage
              (connector_end)
              (connector_end))
            (interface_usage 'EngineToTransmissionInterface' 'engineToTransmissionInterface'
              (connector_end)
              (connector_end))
            (interface_usage 'FuelInterface' 'fuelInterface'
              (connector_end)
              (connector_end))
            (allocation_usage
              (connector_end)
              (connector_end))
            (binding_as_usage
              (connector_end)
              (connector_end))
            (connection_usage
              (connector_end)
              (connector_end))
            (connection_usage
              (connector_end)
              (connector_end))
            (binding_as_usage
              (connector_end)
              (connector_end))
            (connection_usage
              (connector_end)
              (connector_end))
            (binding_as_usage
              (connector_end)
              (connector_end))
            (connection_usage
              (connector_end)
              (connector_end))
            (connection_usage
              (connector_end)
              (connector_end))
            (binding_as_usage
              (connector_end)
              (connector_end))
            (binding_as_usage
              (connector_end)
              (connector_end))
            (malformed)))
        (package_def 'ActionTree'
          (action_usage 'providePower' : 'ProvidePower'
            (item_usage in 'fuelCmd' : 'FuelCmd' :>> 'pwrCmd')
            (default_ref_usage out 'wheelToRoadTorque' :>> 'wheelToRoadTorque' multiplicity value)
            (action_usage 'generateTorque' : 'GenerateTorque'
              (malformed)
              (malformed))
            (action_usage 'amplifyTorque' : 'AmplifyTorque')
            (action_usage 'transferTorque' : 'TransferTorque')
            (action_usage 'distributeTorque' : 'DistributeTorque')
            (line_comment)
            (flow_usage 'generateToAmplify'
              (connector_end)
              (connector_end))
            (line_comment)
            (flow_usage 'amplifyTorque')
            (flow_usage 'transferTorque'))
          (action_usage 'performSelfTest' : 'PerformSelfTest')
          (action_usage 'applyParkingBrake' : 'ApplyParkingBrake')
          (action_usage 'senseTemperature' : 'SenseTemperature'))
        (package_def 'DiscreteInteractions'
          (package_def 'Sequence'
            (part_def 'Driver'
              (port_usage 'p1')
              (port_usage 'p2'))
            (part_usage 'part0'
              (perform_action 'startVehicle'
                (action_usage 'turnVehicleOn')
                (send_node)
                (action_usage 'trigger1')
                (accept_node)
                (flow_usage 'of')
                (action_usage 'startEngine'
                  (item_usage in 'ignitionCmd' : 'IgnitionCmd')
                  (item_usage out 'es' : 'EngineStatus'))
                (flow_usage 'of')
                (action_usage 'sendStatus')
                (send_node)
                (action_usage 'trigger2')
                (accept_node))
              (part_usage 'driver' : 'Driver'
                (perform_action :>> 'startVehicle.turnVehicleOn')
                (perform_action :>> 'startVehicle.trigger2')
                (event_occurrence 'driverReady'))
              (part_usage 'vehicle' : 'Vehicle'
                (perform_action :>> 'startVehicle.trigger1')
                (perform_action :>> 'startVehicle.sendStatus')
                (event_occurrence 'doorClosed'))
              (succession_as_usage
                (connector_end)
                (connector_end))
              (message_usage 'of')
              (message_usage 'of')))
          (occurrence_usage 'CruiseControl1'
            (part_usage 'vehicle_b' :> 'PartsTree::vehicle_b'
              (port_usage :>> 'setSpeedPort'
                (event_occurrence 'setSpeedReceived'))
              (part_usage :>> 'speedSensor'
                (port_usage :>> 'speedSensorPort'
                  (event_occurrence 'sensedSpeedSent')))
              (part_usage :>> 'vehicleSoftware'
                (part_usage :>> 'vehicleController'
                  (part_usage :>> 'cruiseController'
                    (port_usage :>> 'setSpeedPort'
                      (line_comment)
                      (event_occurrence 'setSpeedReceived' value))
                    (port_usage :>> 'speedSensorPort'
                      (event_occurrence 'sensedSpeedReceived'))
                    (port_usage :>> 'cruiseControlPort'
                      (event_occurrence 'fuelCmdSent')))))
              (part_usage :>> 'engine'
                (port_usage :>> 'fuelCmdPort'
                  (event_occurrence 'fuelCmdReceived')))
              (message_usage 'sendSensedSpeed' : 'SensedSpeed'
                (connector_end)
                (connector_end))
              (message_usage 'sendFuelCmd' : 'FuelCmd'
                (connector_end)
                (connector_end))))
          (occurrence_usage 'CruiseControl2'
            (part_usage 'vehicle_b' :> 'PartsTree::vehicle_b'
              (port_usage :>> 'setSpeedPort'
                (event_occurrence 'setSpeedReceived'))
              (part_usage :>> 'speedSensor'
                (port_usage :>> 'speedSensorPort'
                  (malformed)))
              (part_usage :>> 'vehicleSoftware'
                (part_usage :>> 'vehicleController'
                  (part_usage :>> 'cruiseController'
                    (port_usage :>> 'setSpeedPort'
                      (line_comment)
                      (event_occurrence 'setSpeedReceived' value))
                    (port_usage :>> 'speedSensorPort'
                      (event_occurrence 'setSpeedReceived' value)
                      (source_succession
                        (malformed)))
                    (port_usage :>> 'cruiseControlPort'
                      (malformed)))))
              (part_usage :>> 'engine'
                (port_usage :>> 'fuelCmdPort'
                  (malformed)))
              (message_usage 'sendSensedSpeed' : 'SensedSpeed')
              (message_usage 'sendFuelCmd' : 'FuelCmd'))))
        (package_def 'Requirements'
          (import_decl public 'RequirementDerivation::*')
          (import_decl public 'ModelingMetadata::*')
          (line_comment)
          (item_usage 'marketSurvey')
          (dependency from 'vehicleSpecification' to 'marketSurvey')
          (requirement_usage 'vehicleSpecification'
            (sysml_decl 'vehicle' : 'Vehicle')
            (requirement_usage 'vehicleMassRequirement' : 'MassRequirement'
              (documentation)
              (attribute_usage :>> 'massRequired' value)
              (attribute_usage :>> 'massActual' value)
              (attribute_usage 'fuelMassActual' :> 'ISQ::mass')
              (attribute_usage 'fuelMassMax' :> 'ISQ::mass' value)
              (sysml_decl
                (result_expr_member)))
            (allocation_usage
              (connector_end)
              (connector_end))
            (requirement_usage 'vehicleFuelEconomyRequirements'
              (documentation)
              (attribute_usage 'assumedCargoMass' :> 'ISQ::mass')
              (requirement_usage 'cityFuelEconomyRequirement' : 'FuelEconomyRequirement'
                (default_ref_usage :>> 'requiredFuelEconomy' value)
                (sysml_decl
                  (result_expr_member)))
              (requirement_usage 'highwayFuelEconomyRequirement' : 'FuelEconomyRequirement'
                (default_ref_usage :>> 'requiredFuelEconomy' value)
                (sysml_decl
                  (result_expr_member))
                (line_comment)
                (line_comment)
                (metadata_feature typed 'StatusInfo'
                  (feature_def 'status' value)
                  (feature_def 'originator' value)
                  (feature_def 'owner' value)))))
          (requirement_usage 'engineSpecification'
            (sysml_decl 'engine1' : 'Engine')
            (requirement_usage 'engineMassRequirement' : 'MassRequirement'
              (documentation)
              (attribute_usage :>> 'massRequired' value)
              (attribute_usage :>> 'massActual' value))
            (requirement_usage 'torqueGenerationRequirement' : 'TorqueGenerationRequirement'
              (sysml_decl 'generateTorque' value))
            (requirement_usage 'drivePowerOutputRequirement' : 'DrivePowerOutputRequirement'
              (port_usage 'torqueOutPort'
                (default_ref_usage out 'torque' : 'Torque'))))
          (line_comment)
          (malformed)
          (malformed)))
      (package_def 'Engine4Cyl_Variant'
        (import_decl public 'ModelingMetadata::*')
        (line_comment)
        (part_usage 'engine' : 'Engine'
          (part_usage 'cylinders' : 'Cylinder' multiplicity ordered))
        (part_usage 'engine4Cyl' :> 'engine'
          (part_usage :>> 'cylinders' multiplicity)
          (part_usage 'cylinder1' :> 'cylinders' multiplicity)
          (part_usage 'cylinder2' :> 'cylinders' multiplicity)
          (part_usage 'cylinder3' :> 'cylinders' multiplicity)
          (part_usage 'cylinder4' :> 'cylinders' multiplicity))
        (dependency from 'engine4Cyl' to 'VehicleConfiguration_b::PartsTree::vehicle_b::engine'))
      (package_def 'WheelHubAssemblies'
        (line_comment)
        (part_usage 'wheelHubAssy1'
          (part_usage 'wheel1' : 'Wheel'
            (port_usage :>> 'lugNutCompositePort' : 'LugNutCompositePort'
              (port_usage 'lugNutPort' :>> 'lugNutPort' multiplicity)))
          (part_usage 'hub1' : 'Hub'
            (port_usage :>> 'shankCompositePort' : 'ShankCompositePort'
              (port_usage 'shankPort' :>> 'shankPort' multiplicity)))
          (interface_usage 'WheelHubInterface' 'wheelHubInterface'
            (connector_end)
            (connector_end)))
        (line_comment)
        (part_usage 'wheelHubAssy2'
          (part_usage 'wheel1' : 'Wheel'
            (port_usage :>> 'lugNutCompositePort' : 'LugNutCompositePort'
              (port_usage 'lugNutPort' :>> 'lugNutPort' multiplicity)))
          (part_usage 'hub1' : 'Hub'
            (port_usage :>> 'shankCompositePort' : 'ShankCompositePort'
              (port_usage 'shankPort' :>> 'shankPort' multiplicity)))
          (interface_usage 'WheelHubInterface' 'wheelHubInterface'
            (connector_end)
            (connector_end)
            (interface_usage :> 'wheelFastenerInterface' 'wheelFastenerInterface1'
              (connector_end)
              (connector_end))))
        (line_comment)
        (part_usage 'wheelHubAssy3'
          (part_usage 'wheel1' : 'Wheel'
            (port_usage 'lugNutCompositePort' :>> 'lugNutCompositePort'
              (port_usage 'lugNutPort' :>> 'lugNutPort' multiplicity
                (attribute_usage :>> 'threadDia' value)
                (attribute_usage :>> 'threadPitch' value))
              (port_usage 'lugNutPort1' :> 'lugNutPort' multiplicity)
              (port_usage 'lugNutPort2' :> 'lugNutPort' multiplicity)
              (port_usage 'lugNutPort3' :> 'lugNutPort' multiplicity)))
          (part_usage 'hub1' : 'Hub'
            (port_usage 'shankCompositePort' :>> 'shankCompositePort'
              (port_usage 'shankPort' :>> 'shankPort' multiplicity
                (attribute_usage :>> 'threadDia' value)
                (attribute_usage :>> 'threadPitch' value)
                (attribute_usage :>> 'shaftLength' value))
              (port_usage 'shankPort1' :> 'shankPort' multiplicity)
              (port_usage 'shankPort2' :> 'shankPort' multiplicity)
              (port_usage 'shankPort3' :> 'shankPort' multiplicity)))
          (interface_usage 'WheelHubInterface' 'wheelHubInterface'
            (connector_end)
            (connector_end)
            (interface_usage :> 'wheelFastenerInterface' 'wheelFastenerInterface1'
              (connector_end)
              (connector_end)
              (attribute_usage :>> 'maxTorque' value))
            (interface_usage :> 'wheelFastenerInterface' 'wheelFastenerInterface2'
              (connector_end)
              (connector_end)
              (attribute_usage :>> 'maxTorque' value))
            (interface_usage :> 'wheelFastenerInterface' 'wheelFastenerInterface3'
              (connector_end)
              (connector_end)
              (attribute_usage :>> 'maxTorque' value))))))
    (package_def 'VehicleAnalysis'
      (import_decl public 'RiskMetadata::*')
      (import_decl public 'RiskLevelEnum::*')
      (line_comment)
      (import_decl public 'VehicleConfigurations::VehicleConfiguration_b::**')
      (package_def 'FuelEconomyAnalysisModel'
        (import_decl public 'SampledFunctions::SampledFunction')
        (comment)
        (attribute_def 'Scenario' :> 'SampledFunction'
          (attribute_usage 'wayPoint' multiplicity
            (attribute_usage 'elapseTime' :> 'ISQ::time' multiplicity)
            (attribute_usage 'position' :> 'ISQ::distance' multiplicity)))
        (calc_def 'FuelConsumption'
          (default_ref_usage in 'bestFuelConsumption' : 'Real')
          (default_ref_usage in 'idlingFuelConsumption' : 'Real')
          (default_ref_usage in 'tpd_avg' :> 'timePerDistance')
          (attribute_usage 'f' value)
          (return_member))
        (calc_def 'AverageTravelTimePerDistance'
          (default_ref_usage in 'scenario' : 'Scenario')
          (return_member))
        (calc_def 'TraveledDistance'
          (default_ref_usage in 'scenario' : 'Scenario')
          (return_member))
        (calc_def 'IdlingFuelConsumptionPerTime'
          (default_ref_usage in 'engine' : 'Engine')
          (attribute_usage 'idlingFuelConsumptionPerDisplacement' : 'Real' value)
          (return_member))
        (attribute_usage 'specificGravityOfGasoline' : 'Real' value)
        (calc_def 'BestFuelConsumptionPerDistance'
          (default_ref_usage in 'mass' : 'MassValue')
          (default_ref_usage in 'bsfc' : 'Real')
          (default_ref_usage in 'tpd_avg' :> 'timePerDistance')
          (default_ref_usage in 'distance' :> 'length')
          (attribute_usage 'required_power_avg' :> 'ISQ::power')
          (constraint_usage
            (result_expr_member))
          (return_member))
        (calc_def 'ComputeBSFC'
          (default_ref_usage in 'engine' : 'Engine')
          (return_member))
        (sysml_decl 'fuelEconomyAnalysis'
          (sysml_decl value)
          (objective_member)
          (attribute_usage in 'scenario' : 'Scenario')
          (line_comment)
          (attribute_usage 'distance' value)
          (attribute_usage 'tpd_avg' value)
          (attribute_usage 'bsfc' value)
          (attribute_usage 'f_a' value)
          (attribute_usage 'f_b' value)
          (return_member)))
      (package_def 'ElectricalPowerAnalysis')
      (package_def 'ReliabilityAnalyis')
      (package_def 'VehicleTradeOffAnalysis'
        (comment)
        (metadata_feature typed 'Rationale' about 'engineTradeOffAnalysis::vehicle_b_engine4cyl'
          (feature_def 'explanation' value)
          (feature_def 'text' value))
        (line_comment)
        (metadata_feature typed 'Risk' about 'engineTradeOffAnalysis::vehicle_b_engine4cyl'
          (feature_def 'totalRisk' value)
          (feature_def 'technicalRisk' value)
          (feature_def 'scheduleRisk' value)
          (feature_def 'costRisk' value))
        (metadata_feature typed 'Risk' about 'engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::fuelEfficiency'
          (feature_def 'technicalRisk'
            (feature_def 'probability' value)
            (feature_def 'impact' value)))
        (import_decl public 'TradeStudies::*')
        (line_comment)
        (calc_def 'EngineEvaluation'
          (default_ref_usage in 'engineMass' :> 'ISQ::mass')
          (default_ref_usage in 'enginePower' :> 'ISQ::power')
          (default_ref_usage in 'engineFuelEfficiency' : 'Real')
          (default_ref_usage in 'engineCost' : 'Real')
          (return_member))
        (calc_def 'EngineEvaluation_4cyl'
          (default_ref_usage in 'engineMass' :> 'ISQ::mass')
          (default_ref_usage in 'enginePower' :> 'ISQ::power')
          (default_ref_usage in 'engineFuelEfficiency' : 'Real')
          (default_ref_usage in 'engineCost' : 'Real')
          (return_member))
        (calc_def 'EngineEvaluation_6cyl'
          (default_ref_usage in 'engineMass' :> 'ISQ::mass')
          (default_ref_usage in 'enginePower' :> 'ISQ::power')
          (default_ref_usage in 'engineFuelEfficiency' : 'Real')
          (default_ref_usage in 'engineCost' : 'Real')
          (return_member))
        (sysml_decl 'engineTradeOffAnalysis' : 'TradeStudy'
          (sysml_decl 'vehicleAlternatives' :> 'vehicle_b' multiplicity)
          (part_usage 'vehicle_b_engine4cyl' :> 'vehicleAlternatives'
            (part_usage 'engine' :>> 'engine'
              (part_usage 'cylinders' :>> 'cylinders' multiplicity)
              (attribute_usage 'mass' :>> 'mass' value)
              (attribute_usage 'peakHorsePower' :>> 'peakHorsePower' value)
              (attribute_usage 'fuelEfficiency' :>> 'fuelEfficiency' value)
              (attribute_usage 'cost' :>> 'cost' value)))
          (part_usage 'vehicle_b_engine6cyl' :> 'vehicleAlternatives'
            (part_usage 'engine' :>> 'engine'
              (part_usage 'cylinders' :>> 'cylinders' multiplicity)
              (attribute_usage 'mass' :>> 'mass' value)
              (attribute_usage 'peakHorsePower' :>> 'peakHorsePower' value)
              (attribute_usage 'fuelEfficiency' :>> 'fuelEfficiency' value)
              (attribute_usage 'cost' :>> 'cost' value)))
          (objective_member)
          (comment)
          (calc_usage :> 'evaluationFunction'
            (part_usage in 'vehicle' :> 'vehicle_b_engine4cyl')
            (return_member))
          (calc_usage :> 'evaluationFunction'
            (part_usage in 'vehicle' :> 'vehicle_b_engine6cyl')
            (return_member))
          (return_member))))
    (package_def 'VehicleVerification'
      (import_decl public 'VehicleConfigurations::VehicleConfiguration_b::**')
      (import_decl public 'VerificationCaseDefinitions::*')
      (import_decl public 'VerificationCases1::*')
      (line_comment)
      (import_decl public 'VerificationCases::*')
      (import_decl public 'VerificationSystem::*')
      (package_def 'VerificationCaseDefinitions'
        (verification_case_def 'MassTest')
        (verification_case_def 'AccelerationTest')
        (verification_case_def 'ReliabilityTest'))
      (package_def 'VerificationCases1'
        (sysml_decl 'massTests' : 'MassTest'
          (sysml_decl 'vehicle_uut' :> 'vehicle_b')
          (sysml_decl 'vehicleVerificationSubSystem_1' value)
          (objective_member)
          (line_comment)
          (metadata_feature typed 'VerificationMethod'
            (feature_def 'kind' value))
          (action_usage 'weighVehicle'
            (default_ref_usage out 'massMeasured' :> 'ISQ::mass'))
          (source_succession
            (action_usage 'evaluatePassFail'
              (default_ref_usage in 'massMeasured' :> 'ISQ::mass')
              (default_ref_usage out 'verdict' value)))
          (flow_usage
            (connector_end)
            (connector_end))
          (return_member)))
      (package_def 'VerificationSystem'
        (part_usage 'verificationContext'
          (perform_action :>> 'massTests')
          (part_usage 'vehicle_UnitUnderTest' :> 'vehicle_b')
          (part_usage 'massVerificationSystem'
            (part_usage 'scale'
              (perform_action :>> 'massTests.weighVehicle'))
            (part_usage 'operator'
              (perform_action :>> 'massTests.evaluatePassFail'))))))
    (package_def 'VehicleIndividuals'
      (individual_usage individual 'a' : 'VehicleRoadContext_1'
        (portion_usage timeslice 't0_t2_a'
          (portion_usage snapshot 't0_a'
            (attribute_usage 't0' :>> 'time' value)
            (portion_usage snapshot 't0_r' : 'Road_1'
              (default_ref_usage :>> 'Road::incline' value)
              (default_ref_usage :>> 'Road::friction' value))
            (portion_usage snapshot 't0_v' : 'Vehicle_1'
              (default_ref_usage :>> 'Vehicle::position' value)
              (default_ref_usage :>> 'Vehicle::velocity' value)
              (default_ref_usage :>> 'Vehicle::acceleration' value)
              (line_comment)
              (portion_usage snapshot 't0_fa' : 'FrontAxleAssembly_1'
                (portion_usage snapshot 't0_leftFront' : 'Wheel_1')
                (portion_usage snapshot 't0_rightFront' : 'Wheel_2'))))
          (portion_usage snapshot 't1_a'
            (attribute_usage 't1' :>> 'time' value)
            (portion_usage snapshot 't1_r' : 'Road_1'
              (default_ref_usage :>> 'Road::incline' value)
              (default_ref_usage :>> 'Road::friction' value))
            (portion_usage snapshot 't1_v' : 'Vehicle_1'
              (default_ref_usage :>> 'Vehicle::position' value)
              (default_ref_usage :>> 'Vehicle::velocity' value)
              (default_ref_usage :>> 'Vehicle::acceleration' value)
              (line_comment)
              (portion_usage snapshot 't1_fa' : 'FrontAxleAssembly_1'
                (portion_usage snapshot 't1_leftFront' : 'Wheel_1')
                (portion_usage snapshot 't1_rightFront' : 'Wheel_2'))))
          (portion_usage snapshot 't2_a'
            (attribute_usage 't2' :>> 'time' value)
            (portion_usage snapshot 't2_r' : 'Road_1'
              (default_ref_usage :>> 'Road::incline' value)
              (default_ref_usage :>> 'Road::friction' value))
            (portion_usage snapshot 't2_v' : 'Vehicle_1'
              (default_ref_usage :>> 'Vehicle::position' value)
              (default_ref_usage :>> 'Vehicle::velocity' value)
              (default_ref_usage :>> 'Vehicle::acceleration' value)
              (line_comment)
              (portion_usage snapshot 't2_fa' : 'FrontAxleAssembly_1'
                (portion_usage snapshot 't2_leftFront' : 'Wheel_1')
                (portion_usage snapshot 't2_rightFront' : 'Wheel_2')))))))
    (package_def 'MissionContext'
      (comment)
      (import_decl public 'VehicleConfigurations::VehicleConfiguration_b::**')
      (import_decl public 'ParametersOfInterestMetadata::moe')
      (import_decl public 'TransportPassengerScenario::*')
      (package_def 'ContextDefinitions'
        (part_def 'MissionContext' :> 'GenericContext::Context')
        (part_def 'Road')
        (part_def 'Driver'
          (port_usage 'handPort' : 'HandPort')
          (exhibit_state 'driverStates'
            (state_usage 'initial')
            (state_usage 'wait')
            (transition_usage)
            (line_comment)
            (transition_usage)
            (line_comment)
            (transition_usage)))
        (part_def 'Passenger')
        (requirement_usage 'transportRequirements')
        (use_case_def 'TransportPassenger'
          (objective_member)
          (sysml_decl 'vehicle' : 'Vehicle')
          (sysml_decl 'environment')
          (sysml_decl 'road')
          (sysml_decl 'driver')
          (sysml_decl 'passenger' multiplicity)
          (include_use_case)
          (include_use_case))
        (sysml_decl 'getInVehicle' : 'GetInVehicle'
          (action_usage 'unlockDoor_in' multiplicity)
          (source_succession
            (action_usage 'openDoor_in'))
          (source_succession
            (action_usage 'enterVehicle'))
          (source_succession
            (action_usage 'closeDoor_in')))
        (use_case_def 'GetInVehicle'
          (sysml_decl 'vehicle' : 'Vehicle')
          (sysml_decl 'driver' multiplicity)
          (sysml_decl 'passenger' multiplicity)
          (sysml_decl
            (result_expr_member)))
        (sysml_decl 'getOutOfVehicle' : 'GetOutOfVehicle'
          (action_usage 'openDoor_out')
          (source_succession
            (action_usage 'exitVehicle'))
          (source_succession
            (action_usage 'closeDoor_out'))
          (source_succession
            (action_usage 'lockDoor_out')))
        (use_case_def 'GetOutOfVehicle'
          (sysml_decl 'vehicle' : 'Vehicle')
          (sysml_decl 'driver' multiplicity)
          (sysml_decl 'passenger' multiplicity)
          (sysml_decl
            (result_expr_member))))
      (package_def 'TransportPassengerScenario'
        (import_decl public 'ContextDefinitions::TransportPassenger')
        (line_comment)
        (sysml_decl 'transportPassenger' : 'TransportPassenger'
          (initial_node start)
          (source_succession
            (action_usage 'a'
              (action_usage 'driverGetInVehicle' :> 'getInVehicle_a' multiplicity)
              (action_usage 'passenger1GetInVehicle' :> 'getInVehicle_a' multiplicity)))
          (source_succession
            (action_usage 'trigger'))
          (accept_node)
          (source_succession
            (action_usage 'b'
              (action_usage 'driveVehicleToDestination')
              (action_usage 'providePower')))
          (source_succession
            (action_usage 'c'
              (action_usage 'driverGetOutOfVehicle' :> 'getOutOfVehicle_a' multiplicity)
              (action_usage 'passenger1GetOutOfVehicle' :> 'getOutOfVehicle_a' multiplicity)))
          (source_succession
            (default_ref_usage 'done')))
        (line_comment)
        (sysml_decl 'transportPassenger_1' : 'TransportPassenger'
          (line_comment)
          (action_usage 'driverGetInVehicle' :> 'getInVehicle_a' multiplicity)
          (action_usage 'passenger1GetInVehicle' :> 'getInVehicle_a' multiplicity)
          (action_usage 'driverGetOutOfVehicle' :> 'getOutOfVehicle_a' multiplicity)
          (action_usage 'passenger1GetOutOfVehicle' :> 'getOutOfVehicle_a' multiplicity)
          (action_usage 'driveVehicleToDestination')
          (action_usage 'providePower')
          (item_def 'VehicleOnSignal')
          (sysml_decl 'join1')
          (sysml_decl 'join2')
          (sysml_decl 'join3')
          (action_usage 'trigger')
          (accept_node)
          (line_comment)
          (initial_node start)
          (source_succession
            (sysml_decl 'fork1'))
          (source_succession
            (default_ref_usage 'driverGetInVehicle'))
          (source_succession
            (default_ref_usage 'passenger1GetInVehicle'))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (line_comment)
          (sysml_decl 'fork2')
          (source_succession
            (default_ref_usage 'driveVehicleToDestination'))
          (source_succession
            (default_ref_usage 'providePower'))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (sysml_decl 'fork3')
          (source_succession
            (default_ref_usage 'driverGetOutOfVehicle'))
          (source_succession
            (default_ref_usage 'passenger1GetOutOfVehicle'))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))
          (succession_as_usage
            (connector_end)
            (connector_end))))
      (part_usage 'missionContext' : 'ContextDefinitions::MissionContext'
        (attribute_usage #'moe' 'transportTime' :> 'ISQ::time')
        (perform_action :>> 'transportPassenger')
        (line_comment)
        (part_usage 'road' : 'ContextDefinitions::Road' value)
        (part_usage 'driver' : 'ContextDefinitions::Driver' value
          (perform_action :>> 'transportPassenger.a.driverGetInVehicle.unlockDoor_in')
          (perform_action :>> 'transportPassenger.a.driverGetInVehicle.openDoor_in')
          (perform_action :>> 'transportPassenger.a.driverGetInVehicle.enterVehicle')
          (perform_action :>> 'transportPassenger.a.driverGetInVehicle.closeDoor_in')
          (perform_action :>> 'transportPassenger.c.driverGetOutOfVehicle.openDoor_out')
          (perform_action :>> 'transportPassenger.c.driverGetOutOfVehicle.exitVehicle')
          (perform_action :>> 'transportPassenger.c.driverGetOutOfVehicle.closeDoor_out')
          (perform_action :>> 'transportPassenger.c.driverGetOutOfVehicle.lockDoor_out')
          (perform_action :>> 'transportPassenger.b.driveVehicleToDestination'))
        (part_usage 'passenger1' : 'ContextDefinitions::Passenger' value
          (perform_action :>> 'transportPassenger.a.passenger1GetInVehicle.unlockDoor_in')
          (perform_action :>> 'transportPassenger.a.passenger1GetInVehicle.openDoor_in')
          (perform_action :>> 'transportPassenger.a.passenger1GetInVehicle.enterVehicle')
          (perform_action :>> 'transportPassenger.a.passenger1GetInVehicle.closeDoor_in')
          (perform_action :>> 'transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out')
          (perform_action :>> 'transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle')
          (perform_action :>> 'transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out')
          (perform_action :>> 'transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out'))
        (part_usage 'vehicle_b_1' :> 'vehicle_b' value
          (attribute_usage :>> 'position3dVector' value)
          (perform_action :>> 'transportPassenger.b.providePower')
          (default_ref_usage :>> 'providePower')
          (perform_action :>> 'transportPassenger.trigger'))
        (connection_usage
          (connector_end)
          (connector_end))
        (connection_usage
          (connector_end)
          (connector_end))))
    (package_def 'VehicleSuperSetModel'
      (comment)
      (package_def 'VariationPointDefinitions'
        (part_def variation 'TransmissionChoices' :> 'Transmission'
          (variant_usage
            (part_usage 'transmissionAutomatic' : 'TransmissionAutomatic'))
          (variant_usage
            (part_usage 'transmissionManual' : 'TransmissionManual'))))
      (package_def 'VehiclePartsTree'
        (import_decl public 'VariationPointDefinitions::*')
        (part_usage abstract 'vehicleFamily'
          (line_comment)
          (part_usage variation 'engine' : 'Engine'
            (variant_usage
              (part_usage 'engine4Cyl' : 'Engine4Cyl'))
            (variant_usage
              (part_usage 'engine6Cyl' : 'Engine6Cyl'
                (part_usage 'cylinder' : 'Cylinder' multiplicity
                  (attribute_usage variation 'diameter' : 'LengthValue'
                    (variant_usage
                      (attribute_usage 'smallDiameter' : 'LengthValue'))
                    (variant_usage
                      (attribute_usage 'largeDiagmeter' : 'LengthValue')))))))
          (line_comment)
          (part_usage 'transmissionChoices' : 'TransmissionChoices')
          (line_comment)
          (part_usage 'sunroof' : 'Sunroof' multiplicity)
          (line_comment)
          (sysml_decl 'selectionConstraint'
            (result_expr_member))
          (part_usage 'driveshaft')
          (part_usage 'frontAxleAssembly')
          (part_usage 'rearAxleAssembly'))))
    (package_def 'SafetyandSecurityGroups'
      (import_decl public 'VehicleConfigurations::VehicleConfiguration_b::PartsTree::*')
      (package_def 'SafetyGroup'
        (comment)
        (import_decl public 'vehicle_b::**')
        (filter_member
          (classification_expr)))
      (package_def 'SecurityGroup'
        (comment)
        (import_decl public 'vehicle_b::**')
        (filter_member
          (classification_expr)))
      (package_def 'SafetyandSecurityGroup'
        (comment)
        (import_decl public 'vehicle_b::**')
        (filter_member
          (binary_expr)))
      (package_def 'MandatorySafetyGroup'
        (comment)
        (import_decl public 'vehicle_b::**')
        (filter_member
          (binary_expr))))
    (package_def 'Views_Viewpoints'
      (package_def 'ViewpointDefinitions'
        (viewpoint_def 'BehaviorViewpoint')
        (viewpoint_def 'SafetyViewpoint'
          (sysml_decl 'vs' : 'VehicleSafety'))
        (part_def 'SafetyEngineer')
        (concern_def 'VehicleSafety'
          (documentation)
          (sysml_decl)
          (sysml_decl 'se' : 'SafetyEngineer')))
      (package_def 'ViewDefinitions'
        (line_comment)
        (import_decl public 'Views::*')
        (view_def 'TreeView'
          (view_rendering))
        (view_def 'NestedView')
        (view_def 'RelationshipView')
        (view_def 'TableView')
        (view_def 'PartsTreeView' :> 'TreeView'
          (filter_member
            (classification_expr)))
        (view_def 'PartsInterconnection' :> 'NestedView'))
      (package_def 'VehicleViews'
        (import_decl public 'ViewpointDefinitions::*')
        (import_decl public 'ViewDefinitions::*')
        (import_decl public 'VehicleConfigurations::VehicleConfiguration_b::*')
        (sysml_decl 'vehiclePartsTree_Safety' : 'PartsTreeView'
          (sysml_decl 'sv' : 'SafetyViewpoint')
          (expose_member)
          (filter_member
            (classification_expr)))))))
~~~
# FORMAT
~~~sysml
package SimpleVehicleModel{
    // 2023-02 release
    public import Definitions::*;  
    public import ISQ::*;
    package Definitions{
        public import PartDefinitions::*;
        public import PortDefinitions::*;
        public import ItemDefinitions::*;
        public import SignalDefinitions::*;
        public import InterfaceDefinitions::*;
        public import AllocationDefinitions::*;
        public import ActionDefinitions::*;
        public import StateDefinitions::*;
        public import RequirementDefinitions::*;
        public import AttributeDefinitions::*;
        public import IndividualDefinitions::*;
        public import MetadataDefinitions::**;
        public import KeyWord_MetadataDefinitions::*;
        package PartDefinitions{
            part def Vehicle {
                attribute mass :> ISQ::mass;
                attribute dryMass:>ISQ::mass;
                attribute cargoMass:>ISQ::mass;
                attribute position:>ISQ::length;
                attribute velocity:>ISQ::speed;
                attribute acceleration:>ISQ::acceleration;
                attribute electricalPower:>ISQ::power;
                attribute Tmax:>ISQ::temperature;
                attribute maintenanceTime: Time::DateTime; 
                attribute brakePedalDepressed: Boolean;
                port ignitionCmdPort:IgnitionCmdPort;
                port pwrCmdPort:PwrCmdPort;
                port vehicleToRoadPort:VehicleToRoadPort;
                port statusPort:StatusPort;
                perform action providePower;
                perform action provideBraking;
                perform action controlDirection;
                perform action performSelfTest;
                perform action applyParkingBrake;
                perform action senseTemperature;
                exhibit state vehicleStates parallel {
                    ref controller : VehicleController;
                    state operatingStates {
                        entry action initial;
                        state off;                    
                        state starting;                    
                        state on {
                            entry performSelfTest;
                            do providePower;
                            exit applyParkingBrake;
                            constraint {electricalPower<=500[W]}
                        }

                        transition initial then off;

                        transition off_To_starting
                            first off
                            accept ignitionCmd:IgnitionCmd via ignitionCmdPort
                                if ignitionCmd.ignitionOnOff==IgnitionOnOff::on and brakePedalDepressed
                            do send new StartSignal() to controller
                            then starting;
                        
                        transition starting_To_on
                            first starting
                            accept VehicleOnSignal
                            then on;
                        
                        transition on_To_off
                            first on
                            accept VehicleOffSignal
                            do send new OffSignal() to controller
                            then off;
                    }

                    state healthStates {
                        entry action initial;
                        do senseTemperature{
                            out temp;
                        }

                        state normal;
                        state maintenance;
                        state degraded;                    

                        transition initial then normal;

                        transition normal_To_maintenance
                            first normal
                            accept at maintenanceTime
                            then maintenance;

                        transition normal_To_degraded
                            first normal
                            accept when senseTemperature.temp > Tmax 
                            do send new OverTemp() to controller
                            then degraded;

                        transition maintenance_To_normal
                            first maintenance
                            accept ReturnToNormal
                            then normal;

                        transition degraded_To_normal
                            first degraded
                            accept ReturnToNormal
                            then normal;
                    }
                }
            }
            part def Engine{
                attribute mass :> ISQ::mass;
                attribute peakHorsePower:>ISQ::power;
                attribute fuelEfficiency:Real;
                attribute cost:Real;
                attribute displacement :> ISQ::volume;
                port engineControlPort: ~ControlPort;
                port fuelInPort: ~ FuelPort;
                port fuelCmdPort:FuelCmdPort;
                port drivePwrPort:DrivePwrPort;
                port ignitionCmdPort:IgnitionCmdPort;
                port flyWheelPort;
                perform action generateTorque;
                exhibit state engineStates{
                    state off;
                    state starting;
                    state on{
                        do generateTorque;
                    }
                }
            }
            part def StarterMotor{
                port gearPort:GearPort;
            }
            part def Cylinder;
            part def Transmission{
                attribute gearRatio:Real;
                port clutchPort:~DrivePwrPort;
                exhibit state transmissionStates;
            }
            part def Driveshaft;
            part def AxleAssembly;
            part def Axle{
                attribute mass:>ISQ::mass;
            }
            part def FrontAxle:>Axle{
                attribute steeringAngle:>ISQ::angularMeasure;
            }
            part def HalfAxle{
                port shankCompositePort:ShankCompositePort{
                }
            }
            part def Differential;
            part def Wheel{
                attribute diameter:LengthValue;
                port lugNutCompositePort:LugNutCompositePort;
            }
            part def Hub{
                port shankCompositePort:ShankCompositePort;
            }
            abstract part def Software;
            part def VehicleSoftware:>Software;
            part def VehicleController:>Software {
                port controlPort:ControlPort;
                exhibit state controllerStates parallel {
                    state operatingStates {
                        entry action initial; 
                        state off;
                        state on;    
                        transition initial then off;
                        transition 'off-on'
                            first off
                            accept StartSignal
                            then on;
                        transition 'on-off'
                            first on
                            accept OffSignal
                            then off;
                    }
                }  
            }
            part def CruiseController:>Software {
                port setSpeedPort:~SetSpeedPort;
                port speedSensorPort:~SpeedSensorPort;
                port cruiseControlPort:CruiseControlPort;
                exhibit state cruiseControllerStates;
            }
            part def SpeedSensor{
                port speedSensorPort:SpeedSensorPort;
            }
            part def FuelTank{
                attribute mass :> ISQ::mass;
                ref item fuel:Fuel{
                    attribute :>> fuelMass;
                }
                attribute fuelKind:FuelKind;
                attribute fuelMassMax:>ISQ::mass;
                assert constraint fuelConstraint {fuel.fuelMass<=fuelMassMax}
                port fuelOutPort:FuelPort;
                port fuelInPort:~FuelPort;
            }
            part def BodyAssy;
            part def Body{
                attribute color:Colors;
            }
            part def Thermostat;
            part def WaterHose;
            part def Road{
                attribute incline:Real;
                attribute friction:Real;
            }
            part def Engine4Cyl;
            part def Engine6Cyl;
            part def TransmissionChoices;
            part def TransmissionAutomatic;
            part def TransmissionManual;
            part def Sunroof;
            
            //logical Components
            part def ElectricalGenerator;
            part def TorqueGenerator;
            part def SteeringSubsystem;
            part def BrakingSubsystem;
        }
        package PortDefinitions{
            port def IgnitionCmdPort{
                in item ignitionCmd:IgnitionCmd;
            }
            port def StatusPort;
            port def GearPort;
            port def PwrCmdPort{
                in item pwrCmd:PwrCmd;
            }
            port def FuelCmdPort:>PwrCmdPort{
                in item fuelCmd:FuelCmd redefines pwrCmd;
            }
            port def FuelPort{
                out item fuel:Fuel;
            }
            port def DrivePwrPort{
                out torque:Torque;
            }
            port def ShaftPort_a;
            port def ShaftPort_b;
            port def ShaftPort_c;
            port def ShaftPort_d;
            port def DiffPort;
            port def AxlePort;
            port def AxleToWheelPort;
            port def WheelToAxlePort;
            port def WheelToRoadPort;

            port def LugNutCompositePort{
                port lugNutPort:LugNutPort [*];
            }
            port def ShankCompositePort{
                port shankPort:ShankPort [*];
            }
            port def LugNutPort{
                attribute threadDia;
                attribute threadPitch;
            }
            port def ShankPort{
                attribute threadDia;
                attribute threadPitch;   
                attribute shaftLength;
            }
            
            port def VehicleToRoadPort;
            port def ControlPort;
            port def CruiseControlPort:>ControlPort;
            port def SpeedSensorPort;
            port def SetSpeedPort;

            port def DriverCmdPort{
                out item driverCmd[*]:DriverCmd;
            }
            port def HandPort :> DriverCmdPort {
                out item ignitionCmd:IgnitionCmd subsets driverCmd;
                out item pwrCmd:PwrCmd subsets driverCmd;
            }  
        }
        package ItemDefinitions{
            item def PwrCmd{
                attribute throttleLevel:Real;
            }
            item def FuelCmd:>PwrCmd;
            item def Fuel{
                attribute fuelMass:>ISQ::mass;
            }
            item def SensedSpeed{
                attribute speed:>ISQ::speed;
            }
        }
        package SignalDefinitions{
            item def Cmd{
            }
            item def DriverCmd;
            item def IgnitionCmd:>DriverCmd{
                attribute ignitionOnOff:IgnitionOnOff;
            }
            item def EngineStatus;
            
            attribute def VehicleStartSignal;
            attribute def VehicleOnSignal;
            attribute def VehicleOffSignal;
            attribute def StartSignal;
            attribute def OffSignal;
            attribute def OverTemp;
            attribute def ReturnToNormal;
            attribute def SetSpeed:>Real;
        }
        package InterfaceDefinitions{
            interface def EngineToTransmissionInterface{
                end p1:DrivePwrPort;
                end p2:~DrivePwrPort;
                flow p1.torque to p2.torque;
            }
            interface def FuelInterface {
                end fuelOutPort:FuelPort;
                end fuelInPort:~FuelPort;
                flow of Fuel from fuelOutPort.fuel to fuelInPort.fuel;
            }
            
            interface def WheelFastenerInterface{
                end lugNutPort:LugNutPort;
                end shankPort:ShankPort;
                attribute maxTorque : Torque;
                constraint {lugNutPort.threadDia == shankPort.threadDia}
            }
            interface def WheelHubInterface{
                end lugNutCompositePort:LugNutCompositePort;
                end shankCompositePort:ShankCompositePort;
                interface wheelFastenerInterface:WheelFastenerInterface [5]
                    connect lugNutCompositePort.lugNutPort to shankCompositePort.shankPort;
            }
        }
        package AllocationDefinitions{
            allocation def LogicalToPhysical{
                end #logical logicalEnd;
                end #physical physicalEnd;
            }
        }
        package ActionDefinitions{
            action def ProvidePower {
                in item pwrCmd:PwrCmd;
                out wheelToRoadTorque:Torque[2];
            }
            action def GenerateTorque {
                in item fuelCmd:FuelCmd;
                out engineTorque:Torque;
            }
            action def AmplifyTorque {
                in engineTorque:Torque;
                out transmissionTorque:Torque;
            }
            action def TransferTorque {
                in transmissionTorque:Torque;
                out driveshaftTorque:Torque;
            }
            action def DistributeTorque {
                in driveshaftTorque:Torque;
                out wheelToRoadTorque:Torque[2];
            }
            action def PerformSelfTest;
            action def ApplyParkingBrake;
            action def SenseTemperature{
                out temp: ISQ::TemperatureValue;
            }
        }    
        package StateDefinitions {
            state def VehicleStates;
            state def ControllerStates;  
            state def CruiseControllerStates;
        }
        package RequirementDefinitions{
            requirement def MassRequirement{
                doc /*The actual mass shall be less than the required mass*/
                attribute massRequired:>ISQ::mass;
                attribute massActual:>ISQ::mass;
                require constraint {massActual<=massRequired}
            }
            requirement def ReliabilityRequirement{
                doc /*The actual reliability shall be greater than the required reliability*/
                attribute reliabilityRequired:Real;
                attribute reliabilityActual:Real;
                require constraint {reliabilityActual>=reliabilityRequired}
            }
            requirement def TorqueGenerationRequirement {
                doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
                subject generateTorque:ActionDefinitions::GenerateTorque;
            }
            requirement def DrivePowerOutputRequirement { 
                doc /* The engine shall provide a connection point to transfer torque to the transmission.*/
            }
            requirement def FuelEconomyRequirement {
                doc /* The vehicle shall maintain an average fuel economomy of at least x miles per gallon for the nominal 
                driving scenario */
                attribute actualFuelEconomy :> distancePerVolume;
                attribute requiredFuelEconomy :> distancePerVolume;
                require constraint {actualFuelEconomy >= requiredFuelEconomy}
            }
        }
        package AttributeDefinitions{
            public import ScalarValues::*;
            public import Quantities::*;
            public import MeasurementReferences::DerivedUnit;
            public import SIPrefixes::kilo;
            // Numerical Functions provides basic operators such as Sum expression
            public import NumericalFunctions::*;
            public import SI::*;
            public import USCustomaryUnits::*;
            alias Torque for ISQ::TorqueValue;
            
            enum def Colors {black;grey;red;}
            enum def DiameterChoices:>ISQ::LengthValue{
                enum = 60 [mm];
                enum = 80 [mm];
                enum = 100 [mm];
            }
            attribute cylinderDiameter: DiameterChoices = 80 [mm]; 
            enum def IgnitionOnOff {on;off;}
            enum def FuelKind {gas;diesel;}

            distancePerVolume :> scalarQuantities = distance / volume;
            timePerDistance :> scalarQuantities = time / distance;
            volumePerDistance :> scalarQuantities = volume / distance;
            volumePerTime :> scalarQuantities = volume / time;
            
            // kpl is approx .425 * mpg
            kpl : DerivedUnit = km / L;
            rpm : DerivedUnit = 1 / SI::min;
            kW : DerivedUnit = kilo * W;
            
        }
        package IndividualDefinitions{
            individual def VehicleRoadContext_1:>GenericContext::Context;
            individual def Vehicle_1:>Vehicle;
            individual def FrontAxleAssembly_1:>AxleAssembly;
            individual def FrontAxle_1:>FrontAxle;
            individual def Wheel_1:>Wheel;
            individual def Wheel_2:>Wheel;
            individual def RearAxleAssembly_1:>AxleAssembly;
            individual def Road_1:>Road;
        }
        package MetadataDefinitions { 
            public import AnalysisTooling::*;   
            metadata def Safety {
                attribute isMandatory : Boolean;
            }
            metadata def Security;
        }
        package KeyWord_MetadataDefinitions{
            public import Metaobjects::SemanticMetadata;
            
            // the following is used to define the key word failureMode
            state failureModes[*] nonunique;
            
            // with alias <fm>
            metadata def <fm> failureMode :> SemanticMetadata {
                :>> baseType = failureModes meta SysML::StateUsage;
            }
            
            occurrence logicalOccurrences [*] nonunique;
            
            metadata def <l> logical :> SemanticMetadata {
                :>> baseType = logicalOccurrences meta SysML::Usage;
            }
            
            occurrence physicalOccurrences [*] nonunique;
            
            metadata def <p> physical :> SemanticMetadata {
                :>> baseType = physicalOccurrences meta SysML::Usage;
            }  
        }
        package GenericContext {

            part def Context {
                attribute time:TimeValue;
                attribute spatialCF: CartesianSpatial3dCoordinateFrame[1] { :>> mRefs = (m, m, m); }
                attribute velocityCF: CartesianVelocity3dCoordinateFrame[1] = spatialCF/s;
                attribute accelarationCF: CartesianAcceleration3dCoordinateFrame[1] = velocityCF/s;
            }
        }
    }

    package VehicleLogicalConfiguration{
        package PartsTree{
            #logical part vehicleLogical:Vehicle{
                part torqueGenerator:TorqueGenerator{
                    action generateTorque;
                }
                part electricalGenerator:ElectricalGenerator{
                    action generateElectricity;
                }
                part steeringSystem:SteeringSubsystem;
                part brakingSubsystem:BrakingSubsystem;
            }
        }
    }
    package VehicleLogicalToPhysicalAllocation{
        public import VehicleConfigurations::VehicleConfiguration_b::PartsTree::**;
        public import VehicleLogicalConfiguration::PartsTree::*;

        allocation vehicleLogicalToPhysicalAllocation:LogicalToPhysical
            allocate vehicleLogical to vehicle_b{
                allocate vehicleLogical.torqueGenerator to vehicle_b.engine{
                    allocate vehicleLogical.torqueGenerator.generateTorque to vehicle_b.engine.generateTorque;
                }
                allocate vehicleLogical.electricalGenerator to vehicle_b.engine{
                    allocate vehicleLogical.electricalGenerator.generateElectricity to vehicle_b.engine.alternator.generateElectricity;
                }
            }
    } 
    package VehicleConfigurations{
        package VehicleConfiguration_a{
            package PartsTree{
                part vehicle_a:Vehicle{
                    attribute mass redefines Vehicle::mass=dryMass+cargoMass+fuelTank.fuel.fuelMass;
                    attribute dryMass redefines Vehicle::dryMass=sum(partMasses);
                    attribute redefines Vehicle::cargoMass=0 [kg];
                    attribute partMasses [*] nonunique :>ISQ::mass;
                    part fuelTank:FuelTank{
                        attribute redefines mass=75[kg];
                        ref item redefines fuel{
                            attribute redefines fuelMass=50[kg];
                        }   
                    }
                    part frontAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=800[kg];
                        part frontAxle:Axle;
                        part frontWheels:Wheel[2];
                    }
                    part rearAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=875[kg];
                        attribute driveTrainEfficiency:Real = 0.6;
                        part rearAxle:Axle;
                        part rearWheels:Wheel[2]{
                            attribute redefines diameter;
                        }
                    }
                }
            }
            package ActionTree{  
            }
            package Requirements{
            }
        }
        package VehicleConfiguration_b{
            //Shapes library for simple geometry
            public import ShapeItems::Box;
            public import ParametersOfInterestMetadata::mop;
            public import ModelingMetadata::*; // incudes status info
            
            package PartsTree{
                part vehicle_b : Vehicle{
                    #mop attribute mass redefines mass=dryMass+cargoMass+fuelTank.fuel.fuelMass;
                    attribute dryMass redefines dryMass=sum(partMasses);
                    attribute redefines cargoMass default 0 [kg];
                    attribute partMasses=(fuelTank.mass,frontAxleAssembly.mass,rearAxleAssembly.mass,engine.mass,transmission.mass,driveshaft.mass);
                    attribute avgFuelEconomy :> distancePerVolume;
                    port fuelCmdPort: FuelCmdPort redefines pwrCmdPort {
                        in item fuelCmd redefines pwrCmd;
                    }
                    port setSpeedPort:~SetSpeedPort;
                    port vehicleToRoadPort redefines vehicleToRoadPort{
                        port wheelToRoadPort1:WheelToRoadPort;
                        port wheelToRoadPort2:WheelToRoadPort;
                    }
                    perform ActionTree::providePower redefines providePower;
                    perform ActionTree::performSelfTest redefines performSelfTest;
                    perform ActionTree::applyParkingBrake redefines applyParkingBrake;
                    perform ActionTree::senseTemperature redefines senseTemperature;
                    exhibit state vehicleStates redefines vehicleStates;
                    
                    // Example vehicle with simple enveloping shape that is a solid 
                    item :> envelopingShapes : Box[1] {
                        length1:>> length = 4800 [mm];
                        width1:>> width = 1840 [mm];
                        height1:>> height = 1350 [mm];
                    }
                    
                    part fuelTank:FuelTank{
                        attribute redefines mass=75[kg];
                        ref item redefines fuel{
                            attribute redefines fuelMass=60[kg];
                        }
                        attribute redefines fuelMassMax=60 [kg];
                    }
                    part frontAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=800[kg];
                        port shaftPort_d:ShaftPort_d;
                        part frontAxle:FrontAxle;
                        part frontWheels:Wheel[2];
                    }
                    
                    part rearAxleAssembly:AxleAssembly{
                        attribute mass :> ISQ::mass=875[kg];
                        attribute driveTrainEfficiency:Real = 0.6;
                        port shaftPort_d:ShaftPort_d;
                        perform providePower.distributeTorque;
                        part rearWheel1:Wheel{
                            attribute redefines diameter;
                            port wheelToRoadPort:WheelToRoadPort;
                            port lugNutCompositePort :>> lugNutCompositePort{
                                port lugNutPort :>> lugNutPort [5];
                            }
                        }
                        part rearWheel2:Wheel{
                            attribute redefines diameter;
                            port wheelToRoadPort:WheelToRoadPort;
                            port lugNutCompositePort :>> lugNutCompositePort{
                                port lugNutPort :>> lugNutPort [5];
                            }
                        }
                        part differential:Differential{
                            port shaftPort_d:ShaftPort_d;
                            port leftDiffPort:DiffPort;
                            port rightDiffPort:DiffPort;
                        }
                        part rearAxle{
                            part leftHalfAxle:HalfAxle{
                                port leftAxleToDiffPort:AxlePort;
                                port shankCompositePort :>> shankCompositePort{
                                    port shankPort :>> shankPort [5];
                                }
                            }
                            part rightHalfAxle:HalfAxle{
                                port rightAxleToDiffPort:AxlePort;
                                port shankCompositePort :>> shankCompositePort {
                                    port shankPort :>> shankPort [5];
                                }
                            }
                        }
                        
                        bind shaftPort_d=differential.shaftPort_d;
                        connect differential.leftDiffPort to rearAxle.leftHalfAxle.leftAxleToDiffPort;
                        connect differential.rightDiffPort to rearAxle.rightHalfAxle.rightAxleToDiffPort;
                        
                        interface wheelToleftHalAxleInterface:WheelHubInterface 
                            connect [1] rearWheel1.lugNutCompositePort to [1] rearAxle.leftHalfAxle.shankCompositePort;
                        interface wheelTorightHalAxleInterface:WheelHubInterface
                            connect [1] rearWheel2.lugNutCompositePort to [1] rearAxle.rightHalfAxle.shankCompositePort;
                        
                    }
                    part starterMotor:StarterMotor;
                    part engine:Engine{
                        perform providePower.generateTorque redefines generateTorque;            
                        part cylinders:Cylinder[4..6];
                        part alternator{
                            action generateElectricity;
                        }
                        satisfy Requirements::engineSpecification by vehicle_b.engine{
                            requirement torqueGenerationRequirement :>> torqueGenerationRequirement{
                                subject generateTorque redefines generateTorque = vehicle_b.engine.generateTorque;
                            }
                            requirement drivePowerOuputRequirement :>> drivePowerOutputRequirement{
                                port torqueOutPort redefines torqueOutPort=vehicle_b.engine.drivePwrPort;
                            }
                        } 
                    }
                    part transmission:Transmission{
                        attribute mass :> ISQ::mass=100[kg];
                        port shaftPort_a:ShaftPort_a;
                        perform providePower.amplifyTorque;
                    }
                    part driveshaft:Driveshaft{
                        attribute mass :> ISQ::mass=100[kg];
                        port shaftPort_b:ShaftPort_b;
                        port shaftPort_c:ShaftPort_c;
                        perform providePower.transferTorque;
                    }
                    part vehicleSoftware:VehicleSoftware{
                        part vehicleController: VehicleController {
                            exhibit state controllerStates redefines controllerStates;
                            part cruiseController:CruiseController;
                        }
                    }
                    part speedSensor:SpeedSensor;
                    
                    // parts in bodyAssy and interioer are marked as safety or security features
                    part bodyAssy:BodyAssy{
                        part body:Body{
                            attribute :>> color = Colors::red;  
                        }
                        part bumper {@Safety{isMandatory = true;}}
                        part keylessEntry {@Security;}
                    }
                    part interior {
                        part alarm {@Security;}
                        part seatBelt[2] {@Safety{isMandatory = true;}}
                        part frontSeat[2];
                        part driverAirBag {@Safety{isMandatory = false;}}
                    }
                    
                    //connections
                    bind engine.fuelCmdPort=fuelCmdPort;

                    interface engineToTransmissionInterface:EngineToTransmissionInterface
                        connect engine.drivePwrPort to transmission.clutchPort;
                
                    interface fuelInterface:FuelInterface
                        connect fuelTank.fuelOutPort to engine.fuelInPort;

                    allocate ActionTree::providePower.generateToAmplify to engineToTransmissionInterface;
                    
                    bind engine.ignitionCmdPort=ignitionCmdPort;
                    connect starterMotor.gearPort to engine.flyWheelPort;
                    connect vehicleSoftware.vehicleController.controlPort to engine.engineControlPort;
                    bind vehicle_b.setSpeedPort = vehicleSoftware.vehicleController.cruiseController.setSpeedPort;
                    connect speedSensor.speedSensorPort to vehicleSoftware.vehicleController.cruiseController.speedSensorPort;
                    bind vehicleSoftware.vehicleController.cruiseController.cruiseControlPort = vehicleSoftware.vehicleController.controlPort;
                    connect transmission.shaftPort_a to driveshaft.shaftPort_b; 
                    connect driveshaft.shaftPort_c to rearAxleAssembly.shaftPort_d;
                    bind rearAxleAssembly.rearWheel1.wheelToRoadPort=vehicleToRoadPort.wheelToRoadPort1;
                    bind rearAxleAssembly.rearWheel2.wheelToRoadPort=vehicleToRoadPort.wheelToRoadPort2;
                    
                    satisfy Requirements::vehicleSpecification by vehicle_b{
                        requirement vehicleMassRequirement:>>vehicleMassRequirement{
                            attribute redefines massActual=vehicle_b.mass;
                            attribute redefines fuelMassActual = vehicle_b.fuelTank.fuel.fuelMass;
                        }
                    }
                }
            }
            package ActionTree{
                action providePower:ProvidePower{
                    in item fuelCmd:FuelCmd redefines pwrCmd;
                    out wheelToRoadTorque redefines wheelToRoadTorque [2] = distributeTorque.wheelToRoadTorque;
                    action generateTorque:GenerateTorque {
                        in item = providePower.fuelCmd;
                    }
                    action amplifyTorque:AmplifyTorque;
                    action transferTorque:TransferTorque;
                    action distributeTorque:DistributeTorque;
                    
                    //named flow
                    flow generateToAmplify from generateTorque.engineTorque to amplifyTorque.engineTorque;
                    //unnamed flows
                    flow amplifyTorque.transmissionTorque to transferTorque.transmissionTorque;
                    flow transferTorque.driveshaftTorque to distributeTorque.driveshaftTorque;
                }
                action performSelfTest: PerformSelfTest;
                action applyParkingBrake: ApplyParkingBrake;
                action senseTemperature: SenseTemperature;
            }                   
            package DiscreteInteractions{
                package Sequence{
                    part def Driver{
                        port p1;
                        port p2;
                    }

                    part part0{
                        perform action startVehicle{
                            action turnVehicleOn send ignitionCmd via driver.p1{
                                in ignitionCmd:IgnitionCmd;
                            }
                            action trigger1 accept ignitionCmd:IgnitionCmd via vehicle.ignitionCmdPort;
                            flow of IgnitionCmd from trigger1.ignitionCmd to startEngine.ignitionCmd;
                            action startEngine{
                                in item ignitionCmd:IgnitionCmd; 
                                out item es:EngineStatus;
                            }
                            flow of EngineStatus from startEngine.es to sendStatus.es;
                            action sendStatus send es via vehicle.statusPort{
                                in es:EngineStatus;
                            }
                            action trigger2 accept es:EngineStatus via driver.p2;
                        }
                        part driver : Driver {
                            perform startVehicle.turnVehicleOn;
                            perform startVehicle.trigger2;
                            event occurrence driverReady;
                        }
                        part vehicle : Vehicle {
                            perform startVehicle.trigger1;
                            perform startVehicle.sendStatus;
                            event occurrence doorClosed;
                        }
                        first vehicle.doorClosed then driver.driverReady;
                        message of ignitionCmd:IgnitionCmd from driver.turnVehicleOn to vehicle.trigger1;  
                        message of es:EngineStatus from vehicle.sendStatus to driver.trigger2;
                    }
                }
                occurrence CruiseControl1{
                    part vehicle_b:>PartsTree::vehicle_b{
                        port redefines setSpeedPort{
                            event occurrence setSpeedReceived;
                        }
                        part redefines speedSensor{
                            port redefines speedSensorPort{
                                event occurrence sensedSpeedSent;
                            }
                        }
                        part redefines vehicleSoftware{
                            part redefines vehicleController{
                                part redefines cruiseController{
                                    port redefines setSpeedPort{
                                        //analagous to gate: event occurrence bound but may not need this since the port is bound
                                        event occurrence setSpeedReceived = vehicle_b.setSpeedPort.setSpeedReceived;
                                    }
                                    port redefines speedSensorPort{
                                        event occurrence sensedSpeedReceived;
                                    }
                                    port redefines cruiseControlPort{
                                        event occurrence fuelCmdSent;
                                    }
                                }
                            }
                        }
                        part redefines engine{
                            port redefines fuelCmdPort{
                                event occurrence fuelCmdReceived;
                            }
                        }
                        message sendSensedSpeed of SensedSpeed
                            from speedSensor.speedSensorPort.sensedSpeedSent to vehicleSoftware.vehicleController.cruiseController.speedSensorPort.sensedSpeedReceived;
                        message sendFuelCmd of FuelCmd
                            from vehicleSoftware.vehicleController.cruiseController.cruiseControlPort.fuelCmdSent to engine.fuelCmdPort.fuelCmdReceived;
                    }
                }
                occurrence CruiseControl2{
                    part vehicle_b:>PartsTree::vehicle_b{
                        port redefines setSpeedPort{
                            event occurrence setSpeedReceived;
                        }
                        part redefines speedSensor{
                            port redefines speedSensorPort{
                                event sendSensedSpeed.sourceEvent;
                            }
                        }
                        part redefines vehicleSoftware{
                            part redefines vehicleController{
                                part redefines cruiseController{
                                    port redefines setSpeedPort{
                                        //analagous to gate: event occurrence bound but may not need this since the port is bound
                                        event occurrence setSpeedReceived = vehicle_b.setSpeedPort.setSpeedReceived;
                                    }
                                    port redefines speedSensorPort{
                                        event occurrence setSpeedReceived=setSpeedPort.setSpeedReceived;
                                        then event sendSensedSpeed.targetEvent;
                                    }
                                    port redefines cruiseControlPort{             
                                        event sendFuelCmd.sourceEvent;
                                    }
                                }
                            }
                        }
                        part redefines engine{
                            port redefines fuelCmdPort{
                                event sendFuelCmd.targetEvent;
                            }
                        }
                        message sendSensedSpeed of SensedSpeed;
                        message sendFuelCmd of FuelCmd;
                    }
                }
            }
            package Requirements{
                public import RequirementDerivation::*;
                public import ModelingMetadata::*; // incudes status info
                item marketSurvey;
                dependency from vehicleSpecification to marketSurvey;
                
                requirement vehicleSpecification{
                    subject vehicle:Vehicle;
                    requirement <'1'> vehicleMassRequirement: MassRequirement {
                        doc /* The total mass of the vehicle shall be less than or equal to the required mass.
                        Assume total mass includes a full tank of gas of 60 kg*/
                        attribute redefines massRequired=2000 [kg];                     
                        attribute redefines massActual default vehicle.dryMass + fuelMassActual;
                        attribute fuelMassActual:>ISQ::mass;
                        attribute fuelMassMax:>ISQ::mass = 60 [kg];
                        assume constraint {fuelMassActual==fuelMassMax}
                    }
                    
                    allocate vehicleMassRequirement to PartsTree::vehicle_b.mass;
                    
                    requirement <'2'> vehicleFuelEconomyRequirements{
                        doc /* fuel economy requirements group */
                        attribute assumedCargoMass:>ISQ::mass;
                        requirement <'2_1'> cityFuelEconomyRequirement:FuelEconomyRequirement{
                            redefines requiredFuelEconomy= 10 [km / L];
                            assume constraint {assumedCargoMass<=500 [kg]}
                        }
                        requirement <'2_2'> highwayFuelEconomyRequirement:FuelEconomyRequirement{
                            redefines requiredFuelEconomy= 12.75 [km / L];
                            assume constraint {assumedCargoMass<=500 [kg]}
                            
                            //StatusInfo is contained in ModelingMetadata library
                            // StatusKind has values for open, closed, tbd, tbr, tbd
                            @StatusInfo {
                                status = StatusKind::closed;     
                                originator = "Bob";
                                owner = "Mary";
                            }
                        }
                    }
                }
                requirement engineSpecification {
                    subject engine1:Engine;
                    requirement <'1'> engineMassRequirement: MassRequirement {
                        doc /* The total mass of the engine shall be less than or equal to the required mass.*/
                        attribute redefines massRequired=200 [kg];                     
                        attribute redefines massActual = engine1.mass;
                    }
                    requirement torqueGenerationRequirement : TorqueGenerationRequirement{
                        subject generateTorque default engine1.generateTorque;
                    }

                    requirement drivePowerOutputRequirement : DrivePowerOutputRequirement{
                        port torqueOutPort{
                            out torque:Torque;
                        }
                    }
                }
                // the engine mass requirement is derived from the vehicle mass requirement
                #derivation connection {
                    end #original ::> vehicleSpecification.vehicleMassRequirement;
                    end #derive ::> engineSpecification.engineMassRequirement;
                }

            }
        }    
        package Engine4Cyl_Variant{
            public import ModelingMetadata::*; // incudes refinement
            part engine:Engine{
                part cylinders:Cylinder[4..8] ordered;
            }
            part engine4Cyl:>engine{
                part redefines cylinders [4];
                part cylinder1 subsets cylinders[1];
                part cylinder2 subsets cylinders[1];
                part cylinder3 subsets cylinders[1];
                part cylinder4 subsets cylinders[1];
            }
            #refinement dependency engine4Cyl to VehicleConfiguration_b::PartsTree::vehicle_b::engine;
        }
        package WheelHubAssemblies{
            // alternative 1 - w/o explicit nesxted interfaces
            part wheelHubAssy1{
                part wheel1:Wheel{
                    port :>>lugNutCompositePort:LugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5];
                    }
                }
                part hub1:Hub{
                    port :>> shankCompositePort:ShankCompositePort {
                        port shankPort :>> shankPort [5];
                    }
                }
                interface wheelHubInterface:WheelHubInterface
                    connect [1] wheel1.lugNutCompositePort to [1] hub1.shankCompositePort;
            }
            // alternative 2 - w multiple nesxted interfaces
            part wheelHubAssy2{
                part wheel1:Wheel{
                    port :>>lugNutCompositePort:LugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5];
                    }
                }
                part hub1:Hub{
                    port :>> shankCompositePort:ShankCompositePort {
                        port shankPort :>> shankPort [5];
                    }
                }
                interface wheelHubInterface:WheelHubInterface
                    connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort {
                        interface wheelFastenerInterface1 :> wheelFastenerInterface
                            connect [5] lugNutPort ::> lugNutCompositePort.lugNutPort to [5] shankPort ::> shankCompositePort.shankPort;
                        }
            }
            // alternative 3 - w explicit nesxted interfaces
            part wheelHubAssy3{
                part wheel1:Wheel{
                    port lugNutCompositePort :>> lugNutCompositePort {
                        port lugNutPort [5] :>> lugNutPort {
                            attribute :>> threadDia = 14 [mm];
                            attribute :>> threadPitch = 1.5 [mm];
                        }
                        port lugNutPort1 [1] :> lugNutPort;
                        port lugNutPort2 [1] :> lugNutPort;
                        port lugNutPort3 [1] :> lugNutPort;
                    }
}
                part hub1:Hub{
                    port shankCompositePort :>> shankCompositePort {
                        port shankPort [5] :>> shankPort {
                            attribute :>> threadDia = 14 [mm];
                            attribute :>> threadPitch = 1.5 [mm];
                            attribute :>> shaftLength = 70 [mm];
                        }
                        port shankPort1 [1] :> shankPort;
                        port shankPort2 [1] :> shankPort;
                        port shankPort3 [1] :> shankPort;
                    }
}
                interface wheelHubInterface:WheelHubInterface
                    connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort {
                        interface wheelFastenerInterface1 :> wheelFastenerInterface
                            connect lugNutPort ::> lugNutCompositePort.lugNutPort1 to shankPort ::> shankCompositePort.shankPort1 {
                                attribute :>> maxTorque = 90 * 1.356 [N*m];
                        }
                        interface wheelFastenerInterface2 :> wheelFastenerInterface
                            connect lugNutPort ::> lugNutCompositePort.lugNutPort2 to shankPort ::> shankCompositePort.shankPort2 {
                                attribute :>> maxTorque = 90 * 1.356 [N*m];
                        }
                        interface wheelFastenerInterface3 :> wheelFastenerInterface
                            connect lugNutPort ::> lugNutCompositePort.lugNutPort3 to shankPort ::> shankCompositePort.shankPort3 {
                                attribute :>> maxTorque = 90 * 1.356 [N*m];
                        }
                }
            }
        }
    }
    package VehicleAnalysis{
        public import RiskMetadata::*;
        public import RiskLevelEnum::*;
        // recursive public import uses double asterisk **
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        package FuelEconomyAnalysisModel{
            public import SampledFunctions::SampledFunction;
              
            /*
            This analysis model was provided by Hisashi Miyashita on January 27, 2021
              We use the simplest fuel consumption analysis model introduced in:
              Akcelik, R. "Fuel efficiency and other objectives in traffic system management." Traffic Engineering and Control 22.2 (1981): 54-65. 

              Fuel consumption rate f can be decomposed to:
              f = f_a + f_b * tpd_avg,
              where tpd_avg is average interrupted travel time per unit distance, actually the inverse of the average velocity [t/km];
              f_a is the best fuel consumption per distance; and
              f_b is the additional fuel consumption per distance and average travel time, which can be regarded as the idling fuel consumption.
              Approximately, it is proportional to engine displacement and it ranges from 0.5 to 0.6 [l/hour/litre of engine displacement]
              according to:
              Review of the Incidence, Energy Use and Costs of Passenger Vehicle Idling; Gordon W. Taylor, P.Eng. Prepared for the Office of Energy Efficiency, Natural Resources Canada, 2003

              We assume f_a can be approximated to
              fuel_consumption / distance = BSFC * SGG * required_power_avg * tpd_avg,
              where required_power_avg is the required power, and it can be approximately derived from:
                  total_energy == P_req * tpd_avg * distance == 1/2 * mass / tpd_avg^2
              This part is computed with BestFuelConsumptionPerDistance calc def.

              BSFC means Brake-Specific Fuel Consumption, defined as gram/power.  SGG is the specific gravity of gasoline.
              The high octane gasoline is about 0.76[l/kg].
            */
            
            attribute def Scenario :> SampledFunction {
                attribute wayPoint[1..*] {
                    attribute elapseTime[1] :> ISQ::time;
                    attribute position[1] :> ISQ::distance;
                }
            }
            
            calc def FuelConsumption {
                in bestFuelConsumption: Real;
                in idlingFuelConsumption: Real; 
                in tpd_avg:>timePerDistance;
                attribute f = bestFuelConsumption + idlingFuelConsumption * tpd_avg;
                return dpv :> distancePerVolume = 1/f;
            }
            
            calc def AverageTravelTimePerDistance {
                in scenario: Scenario;
                return tpd_avg:>timePerDistance;
            }
            calc def TraveledDistance {
                in scenario: Scenario;
                return distance:> length;
            }
            calc def IdlingFuelConsumptionPerTime {
                in engine:Engine;
                attribute idlingFuelConsumptionPerDisplacement: Real = 0.5;
                return f_a : Real = engine.displacement * idlingFuelConsumptionPerDisplacement;
            }

            attribute specificGravityOfGasoline: Real = 0.76;
            calc def BestFuelConsumptionPerDistance {
                in mass: MassValue;
                in bsfc: Real;
                in tpd_avg:> timePerDistance;
                in distance:>length;
                attribute required_power_avg:> ISQ::power;
                constraint {required_power_avg == 1/2 * mass * tpd_avg **(-3) / distance}
                return f_b : Real = bsfc * specificGravityOfGasoline * required_power_avg * tpd_avg;
            }

            calc def ComputeBSFC{
                in engine: Engine;
                return : Real;
            }

            analysis fuelEconomyAnalysis  {    
                subject = vehicle_b; 
                
                objective fuelEconomyAnalysisObjective {
                    doc /*estimate the vehicle fuel economy*/
                    require vehicleSpecification.vehicleFuelEconomyRequirements;
                }
                
                in attribute scenario: Scenario;
                // define a series of waypoints
                
                attribute distance = TraveledDistance(scenario);
                attribute tpd_avg = AverageTravelTimePerDistance(scenario);
                attribute bsfc = ComputeBSFC(vehicle_b.engine);
                attribute f_a = BestFuelConsumptionPerDistance(vehicle_b.mass, bsfc, tpd_avg, distance);
                attribute f_b = IdlingFuelConsumptionPerTime(vehicle_b.engine);

                return attribute calculatedFuelEconomy:>distancePerVolume=FuelConsumption(f_a, f_b, tpd_avg);
            }
        }
        package ElectricalPowerAnalysis{
        }
        package ReliabilityAnalyis{
        }
        package VehicleTradeOffAnalysis{
            /* The following example provides the rationale for selecting the engine4cyl. 
            The rationale and risk are contained in a metadata library. */
            
            @Rationale about engineTradeOffAnalysis::vehicle_b_engine4cyl{
                explanation = VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis;          
                text = "the engine4cyl was evaluated to have a higher objective function compared to the engine6cyl based on the trade-off analyiss"; 
            }
            
            // The following risk for the engine4cyl could have been included as part of the objective evaluaiton criteria
            
            @Risk about engineTradeOffAnalysis::vehicle_b_engine4cyl {
                totalRisk = medium;
                technicalRisk = medium;
                scheduleRisk = medium;
                costRisk = RiskLevelEnum::low;
            }
            @Risk about engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::fuelEfficiency {
                technicalRisk {
                    probability = 0.3;
                    impact = 0.5;
                }
            }
            
                
            public import TradeStudies::*;
            //evaluation function with criterion engine mass, engine power, and engine cost
            calc def EngineEvaluation {
                in engineMass:>ISQ::mass;
                in enginePower:>ISQ::power; 
                in engineFuelEfficiency:Real;
                in engineCost:Real;
                return eval:Real;
            }
            calc def EngineEvaluation_4cyl {
                in engineMass:>ISQ::mass;
                in enginePower:>ISQ::power;
                in engineFuelEfficiency:Real;
                in engineCost:Real;
                return eval:Real;
            }
            calc def EngineEvaluation_6cyl {
                in engineMass:>ISQ::mass;
                in enginePower:>ISQ::power;
                in engineFuelEfficiency:Real;
                in engineCost:Real;
                return eval:Real;
            }
            analysis engineTradeOffAnalysis:TradeStudy{
                subject vehicleAlternatives[2]:>vehicle_b;   
                
                part vehicle_b_engine4cyl:>vehicleAlternatives{   
                    part engine redefines engine{
                        part cylinders :>> cylinders [4];
                        attribute mass redefines mass=180 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 180 [W];
                        attribute fuelEfficiency redefines fuelEfficiency=.6;
                        attribute cost redefines cost = 1000;                     
                    }
                }
                part vehicle_b_engine6cyl:>vehicleAlternatives{   
                    part engine redefines engine{  
                        part cylinders redefines cylinders [6];
                        attribute mass redefines mass=220 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 220 [W];
                        attribute fuelEfficiency redefines fuelEfficiency=.5;
                        attribute cost redefines cost = 1500;
                    }
                }
                
                objective :MaximizeObjective;
                    /*Select vehicle alternative with the engine whose evaluation function returns the max value*/
                
                calc :> evaluationFunction{
                    in part vehicle:>vehicle_b_engine4cyl;
                    return attribute eval:Real=EngineEvaluation_4cyl (vehicle.engine.mass, vehicle.engine.peakHorsePower, vehicle.engine.fuelEfficiency, vehicle.engine.cost); 
                }
                calc :> evaluationFunction{
                    in part vehicle:>vehicle_b_engine6cyl;
                    return attribute eval:Real=EngineEvaluation_6cyl (vehicle.engine.mass, vehicle.engine.peakHorsePower, vehicle.engine.fuelEfficiency, vehicle.engine.cost); 
                }                                                  
                return part selectedVehicle:>vehicle_b;
            }
        }
    }
    package VehicleVerification{
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        public import VerificationCaseDefinitions::*;
        public import VerificationCases1::*;
        // the following is a model library which contains VerdictKind
        public import VerificationCases::*;
        public import VerificationSystem::*;
        package VerificationCaseDefinitions{
            verification def MassTest;
            verification def AccelerationTest;
            verification def ReliabilityTest;
        }
        package VerificationCases1{
            verification massTests:MassTest {
                subject vehicle_uut :> vehicle_b;
                actor vehicleVerificationSubSystem_1 = verificationContext.massVerificationSystem;
                objective {
                    verify vehicleSpecification.vehicleMassRequirement{
                        redefines massActual=weighVehicle.massMeasured;
                    }
                }     
                // method kinds are test, demo, analyze, should also include inspection, similarity
               @ VerificationMethod{
                    kind = (VerificationMethodKind::test, VerificationMethodKind::analyze);
                }
                action weighVehicle {
                    out massMeasured:>ISQ::mass;
                }
                then action evaluatePassFail {
                    in massMeasured:>ISQ::mass;
                    out verdict = PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut));
                }
                flow from weighVehicle.massMeasured to evaluatePassFail.massMeasured;
                return :>> verdict = evaluatePassFail.verdict;
            }
        }
        package VerificationSystem{
            part verificationContext{
                perform massTests;
                part vehicle_UnitUnderTest :> vehicle_b;
                part massVerificationSystem{
                    part scale{
                        perform massTests.weighVehicle;
                    }
                    part operator{
                        perform massTests.evaluatePassFail;
                    }
                }
            }
        }
    }
    package VehicleIndividuals{
        individual a:VehicleRoadContext_1{
            timeslice t0_t2_a{
                snapshot t0_a {             
                    attribute t0 redefines time=0 [s];
                    snapshot t0_r:Road_1{
                        :>>Road::incline =0;
                        :>>Road::friction=.1;
                    }
                    snapshot t0_v:Vehicle_1{
                        :>>Vehicle::position=0 [m];
                        :>>Vehicle::velocity=0 [m];
                        :>>Vehicle::acceleration=1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t0_fa:FrontAxleAssembly_1{
                            snapshot t0_leftFront:Wheel_1;
                            snapshot t0_rightFront:Wheel_2;
                        }
                    }
                }
                snapshot t1_a{
                    attribute t1 redefines time=1 [s];
                    snapshot t1_r:Road_1{
                        :>>Road::incline =0;
                        :>>Road::friction=.1;
                    }
                    snapshot t1_v:Vehicle_1{
                        :>>Vehicle::position=.98 [m];
                        :>>Vehicle::velocity=1.96 [m/s];
                        :>>Vehicle::acceleration=1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t1_fa:FrontAxleAssembly_1{
                            snapshot t1_leftFront:Wheel_1;
                            snapshot t1_rightFront:Wheel_2;
                        }
                    }
                }
                snapshot t2_a{
                    attribute t2 redefines time=2 [s];
                    snapshot t2_r:Road_1{
                        :>>Road::incline =0;
                        :>>Road::friction=.1;
                    }
                    snapshot t2_v:Vehicle_1{
                        :>>Vehicle::position=3.92 [m];
                        :>>Vehicle::velocity=3.92 [m/s];
                        :>>Vehicle::acceleration=1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t2_fa:FrontAxleAssembly_1{
                            snapshot t2_leftFront:Wheel_1;
                            snapshot t2_rightFront:Wheel_2;
                        }
                    }
                }
            }
        }
    }
    package MissionContext{
        /* Define mission context with mission use cases for vehicle_b */
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        public import ParametersOfInterestMetadata::moe;
        public import TransportPassengerScenario::*;
        package ContextDefinitions{
            part def MissionContext:>GenericContext::Context;
            part def Road;
            part def Driver{
                port handPort:HandPort{
                }
                exhibit state driverStates{
                    state initial;
                    state wait;
                    transition initial then wait;
                    //ignition on
                    transition 'wait-wait-1'
                        first wait
                        do send new IgnitionCmd (ignitionOnOff=IgnitionOnOff::on) via handPort
                        then wait;
                    // ignition off
                    transition 'wait-wait-2'
                        first wait
                        do send new IgnitionCmd (ignitionOnOff=IgnitionOnOff::off) via handPort
                        then wait;
                }
            }
            part def Passenger;
            
            requirement transportRequirements;
            use case def TransportPassenger{
                objective TransportObjective {
                    doc /*deliver passenger to destination safely, comfortably, and within acceptable time*/
                    require transportRequirements;
                }
                subject vehicle:Vehicle;
                actor environment;
                actor road;
                actor driver;
                actor passenger [0..4];
                include use case getInVehicle_a:>getInVehicle [1..5];
                include use case getOutOfVehicle_a:>getOutOfVehicle [1..5];
            }
            
            use case getInVehicle:GetInVehicle {
                action unlockDoor_in [0..1];
                then action openDoor_in;
                then action enterVehicle;
                then action closeDoor_in;
            }
            use case def GetInVehicle{
                subject vehicle:Vehicle;
                actor driver [0..1];
                actor passenger [0..1];
                assert constraint {driver != null xor passenger != null}
            }

            use case getOutOfVehicle:GetOutOfVehicle {
                action openDoor_out;
                then action exitVehicle;
                then action closeDoor_out;
                then action lockDoor_out;
            }
            use case def GetOutOfVehicle{
                subject vehicle:Vehicle;
                actor driver [0..1];
                actor passenger [0..1];
                assert constraint {driver != null xor passenger != null}
            }
        }
        package TransportPassengerScenario{
            public import ContextDefinitions::TransportPassenger;
            
            // this version uses nesting vs fork and join for concurrent actions
            use case transportPassenger:TransportPassenger{
                first start; 
                then action a{
                    action driverGetInVehicle subsets getInVehicle_a[1];
                    action passenger1GetInVehicle subsets getInVehicle_a[1];
                }
                then action trigger accept ignitionCmd:IgnitionCmd;
                then action b{
                    action driveVehicleToDestination;
                    action providePower;   
                }
                then action c{
                    action driverGetOutOfVehicle subsets getOutOfVehicle_a[1];
                    action passenger1GetOutOfVehicle subsets getOutOfVehicle_a[1];
                }
                then done;
            }
            
            
            //this version uses forks and joins
            use case transportPassenger_1:TransportPassenger{
                // declare actions
                action driverGetInVehicle subsets getInVehicle_a[1];
                action passenger1GetInVehicle subsets getInVehicle_a[1];
                action driverGetOutOfVehicle subsets getOutOfVehicle_a[1];
                action passenger1GetOutOfVehicle subsets getOutOfVehicle_a[1];
                action driveVehicleToDestination;
                action providePower;
                item def VehicleOnSignal;
                join join1;
                join join2;
                join join3;
                action trigger accept ignitionCmd:IgnitionCmd;
                
                // define control flow
                first start;               
                then fork fork1;
                    then driverGetInVehicle;
                    then passenger1GetInVehicle;
                first driverGetInVehicle then join1;
                first passenger1GetInVehicle then join1;
                first join1 then trigger;
                first trigger then fork2;
                //succession trigger if trigger.ignitionCmd.ignitionOnOff==IgnitionOnOff::on then fork2;
                
                fork fork2;
                    then driveVehicleToDestination;
                    then providePower;
                first driveVehicleToDestination then join2;
                first providePower then join2;
                first join2 then fork3;

                fork fork3; 
                    then driverGetOutOfVehicle;
                    then passenger1GetOutOfVehicle;
                first driverGetOutOfVehicle then join3;
                first passenger1GetOutOfVehicle then join3;

                first join3 then done;
            }
        }
        
        part missionContext:ContextDefinitions::MissionContext{
            #moe attribute transportTime :> ISQ::time;
            perform transportPassenger;
            // bind parts to actors of use case
            part road:ContextDefinitions::Road = transportPassenger.road;
            part driver:ContextDefinitions::Driver = transportPassenger.driver{
                perform transportPassenger.a.driverGetInVehicle.unlockDoor_in;
                perform transportPassenger.a.driverGetInVehicle.openDoor_in;
                perform transportPassenger.a.driverGetInVehicle.enterVehicle; 
                perform transportPassenger.a.driverGetInVehicle.closeDoor_in;
                perform transportPassenger.c.driverGetOutOfVehicle.openDoor_out;
                perform transportPassenger.c.driverGetOutOfVehicle.exitVehicle; 
                perform transportPassenger.c.driverGetOutOfVehicle.closeDoor_out;
                perform transportPassenger.c.driverGetOutOfVehicle.lockDoor_out;
                perform transportPassenger.b.driveVehicleToDestination;
            }
            part passenger1:ContextDefinitions::Passenger = transportPassenger.passenger {
                perform transportPassenger.a.passenger1GetInVehicle.unlockDoor_in;
                perform transportPassenger.a.passenger1GetInVehicle.openDoor_in;
                perform transportPassenger.a.passenger1GetInVehicle.enterVehicle; 
                perform transportPassenger.a.passenger1GetInVehicle.closeDoor_in;
                perform transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out;
                perform transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle; 
                perform transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out;
                perform transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out;
            }
            part vehicle_b_1:>vehicle_b = transportPassenger.vehicle{
                attribute :>> position3dVector = (0,0,0) [spatialCF];
                perform transportPassenger.b.providePower redefines providePower;
                perform transportPassenger.trigger;
            }
            connect driver.handPort to vehicle_b_1.ignitionCmdPort;
            connect road to vehicle_b_1.vehicleToRoadPort;
        }
    }
    package VehicleSuperSetModel{
        /* all of vehicleFamily is included in the superset model to enable subsetting a specific vehicle configuration*/
        package VariationPointDefinitions {
            variation part def TransmissionChoices:>Transmission {
                variant part transmissionAutomatic:TransmissionAutomatic;
                variant part transmissionManual:TransmissionManual;
            }
        }
        package VehiclePartsTree{
            public import VariationPointDefinitions::*;
            abstract part vehicleFamily {
                // variation with nested variation
                variation part engine:Engine{
                    variant part engine4Cyl:Engine4Cyl;
                    variant part engine6Cyl:Engine6Cyl{
                        part cylinder:Cylinder [6]{
                            variation attribute diameter:LengthValue{
                                variant attribute smallDiameter:LengthValue;
                                variant attribute largeDiagmeter:LengthValue;
                            }
                        }
                    }
                }
                // variation point based on variation of part definition
                part transmissionChoices:TransmissionChoices;
                // optional variation point
                part sunroof:Sunroof[0..1];
                // selection constraint
                assert constraint selectionConstraint{
                    (engine==engine::engine4Cyl and transmissionChoices==TransmissionChoices::transmissionManual) xor
                    (engine==engine::engine6Cyl and transmissionChoices==TransmissionChoices::transmissionAutomatic)
                }
                part driveshaft;
                part frontAxleAssembly;
                part rearAxleAssembly;
            }
        }
    }
    package SafetyandSecurityGroups {
        public import VehicleConfigurations::VehicleConfiguration_b::PartsTree::*;
        package SafetyGroup {
            /* Parts that contribute to safety. */
            public import vehicle_b::**;
            filter @Safety;
        }
        package SecurityGroup {
            /* Parts that contribute to security. */
            public import vehicle_b::**;
            filter @Security;
        }
        package SafetyandSecurityGroup {
            /* Parts that contribute to safety OR security. */
            public import vehicle_b::**;
            filter @Safety or @Security;
        }
        package MandatorySafetyGroup {
            /* Parts that contribute to safety AND are mandatory. */
            public import vehicle_b::**;
            filter @Safety and Safety::isMandatory;
        }
    }
    package Views_Viewpoints{
       package ViewpointDefinitions{
            viewpoint def BehaviorViewpoint;
            viewpoint def SafetyViewpoint{
                frame concern vs:VehicleSafety;
            }
            part def SafetyEngineer;
            concern def VehicleSafety {
                doc /* identify system safety features */
                subject;
                stakeholder se:SafetyEngineer;
            }
        }
        package ViewDefinitions{
            //public import Views to access rendering method library 
            public import Views::*;
            view def TreeView{
                render asTreeDiagram;
            }
            view def NestedView; 
            view def RelationshipView;
            view def TableView;
            view def PartsTreeView:>TreeView {
                filter @SysML::PartUsage;
            }
            view def PartsInterconnection:>NestedView;
        }
        package VehicleViews{
            public import ViewpointDefinitions::*;
            public import ViewDefinitions::*;
            public import VehicleConfigurations::VehicleConfiguration_b::*;
            view vehiclePartsTree_Safety:PartsTreeView{
                satisfy requirement sv:SafetyViewpoint;
                expose PartsTree::**;
                filter @Safety;
            }
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_expression
semantic.duplicate_name 'p1'
semantic.duplicate_name 'amplifyTorque'
semantic.duplicate_name 'transferTorque'
semantic.duplicate_name 'of'
semantic.duplicate_name 'of'
semantic.duplicate_name 'technicalRisk'
semantic.duplicate_name 'driverGetInVehicle'
semantic.duplicate_name 'passenger1GetInVehicle'
semantic.duplicate_name 'driveVehicleToDestination'
semantic.duplicate_name 'providePower'
semantic.duplicate_name 'driverGetOutOfVehicle'
semantic.duplicate_name 'passenger1GetOutOfVehicle'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::temperature'
semantic.unresolved_name 'Time::DateTime'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::volume'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::angularMeasure'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'driverCmd'
semantic.unresolved_name 'driverCmd'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::TemperatureValue'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'CartesianVelocity3dCoordinateFrame'
semantic.unresolved_name 'CartesianAcceleration3dCoordinateFrame'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'envelopingShapes'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Safety'
semantic.unresolved_name 'Security'
semantic.unresolved_name 'Security'
semantic.unresolved_name 'Safety'
semantic.unresolved_name 'Safety'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'StatusInfo'
semantic.unresolved_name 'SampledFunction'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::distance'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'length'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'length'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Rationale'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TradeStudy'
semantic.ambiguous_name 'vehicle_b'
semantic.ambiguous_name 'engine'
semantic.ambiguous_name 'mass'
semantic.ambiguous_name 'engine'
semantic.ambiguous_name 'mass'
semantic.unresolved_name 'MaximizeObjective'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'Real'
semantic.ambiguous_name 'vehicle_b'
semantic.ambiguous_name 'vehicle_b'
semantic.unresolved_name 'VerificationMethod'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'verdict'
semantic.ambiguous_name 'vehicle_b'
semantic.unresolved_name 'massTests::evaluatePassFail'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::unlockDoor_in'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::openDoor_in'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::enterVehicle'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::closeDoor_in'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::openDoor_out'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::exitVehicle'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::closeDoor_out'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::lockDoor_out'
semantic.unresolved_name 'transportPassenger::b::driveVehicleToDestination'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::unlockDoor_in'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::openDoor_in'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::enterVehicle'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::closeDoor_in'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::openDoor_out'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::exitVehicle'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::closeDoor_out'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::lockDoor_out'
semantic.ambiguous_name 'vehicle_b'
semantic.unresolved_name 'position3dVector'
semantic.unresolved_name 'transportPassenger::b::providePower'
semantic.ambiguous_name 'providePower'
semantic.unresolved_name 'transportPassenger::trigger'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_usage_declaration
parse.expected_usage_declaration
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_expression
semantic.duplicate_name 'p1'
semantic.duplicate_name 'amplifyTorque'
semantic.duplicate_name 'transferTorque'
semantic.duplicate_name 'of'
semantic.duplicate_name 'of'
semantic.duplicate_name 'technicalRisk'
semantic.duplicate_name 'driverGetInVehicle'
semantic.duplicate_name 'passenger1GetInVehicle'
semantic.duplicate_name 'driveVehicleToDestination'
semantic.duplicate_name 'providePower'
semantic.duplicate_name 'driverGetOutOfVehicle'
semantic.duplicate_name 'passenger1GetOutOfVehicle'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::temperature'
semantic.unresolved_name 'Time::DateTime'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::volume'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::angularMeasure'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'driverCmd'
semantic.unresolved_name 'driverCmd'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::TemperatureValue'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'CartesianVelocity3dCoordinateFrame'
semantic.unresolved_name 'CartesianAcceleration3dCoordinateFrame'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'envelopingShapes'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Safety'
semantic.unresolved_name 'Security'
semantic.unresolved_name 'Security'
semantic.unresolved_name 'Safety'
semantic.unresolved_name 'Safety'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'StatusInfo'
semantic.unresolved_name 'SampledFunction'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::distance'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'length'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'length'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Rationale'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TradeStudy'
semantic.ambiguous_name 'vehicle_b'
semantic.ambiguous_name 'engine'
semantic.ambiguous_name 'mass'
semantic.ambiguous_name 'engine'
semantic.ambiguous_name 'mass'
semantic.unresolved_name 'MaximizeObjective'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'Real'
semantic.ambiguous_name 'vehicle_b'
semantic.ambiguous_name 'vehicle_b'
semantic.unresolved_name 'VerificationMethod'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'verdict'
semantic.ambiguous_name 'vehicle_b'
semantic.unresolved_name 'massTests::evaluatePassFail'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::unlockDoor_in'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::openDoor_in'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::enterVehicle'
semantic.unresolved_name 'transportPassenger::a::driverGetInVehicle::closeDoor_in'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::openDoor_out'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::exitVehicle'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::closeDoor_out'
semantic.unresolved_name 'transportPassenger::c::driverGetOutOfVehicle::lockDoor_out'
semantic.unresolved_name 'transportPassenger::b::driveVehicleToDestination'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::unlockDoor_in'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::openDoor_in'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::enterVehicle'
semantic.unresolved_name 'transportPassenger::a::passenger1GetInVehicle::closeDoor_in'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::openDoor_out'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::exitVehicle'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::closeDoor_out'
semantic.unresolved_name 'transportPassenger::c::passenger1GetOutOfVehicle::lockDoor_out'
semantic.ambiguous_name 'vehicle_b'
semantic.unresolved_name 'position3dVector'
semantic.unresolved_name 'transportPassenger::b::providePower'
semantic.ambiguous_name 'providePower'
semantic.unresolved_name 'transportPassenger::trigger'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel"))) (name "SimpleVehicleModel") (declared-name "SimpleVehicleModel")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import10"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import11"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import2"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import3"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import4"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import5"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import6"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import7"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import8"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::*#import9"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions"))) (name "ActionDefinitions") (declared-name "ActionDefinitions")
              (contains
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque"))) (name "AmplifyTorque") (declared-name "AmplifyTorque")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque")))))
                  )
                )
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake"))) (name "ApplyParkingBrake") (declared-name "ApplyParkingBrake"))
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque"))) (name "DistributeTorque") (declared-name "DistributeTorque")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque::wheelToRoadTorque"))) (name "wheelToRoadTorque") (declared-name "wheelToRoadTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque")))))
                  )
                )
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque"))) (name "GenerateTorque") (declared-name "GenerateTorque")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque")))))
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque")))))
                  )
                )
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest"))) (name "PerformSelfTest") (declared-name "PerformSelfTest"))
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower"))) (name "ProvidePower") (declared-name "ProvidePower")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (name "pwrCmd") (declared-name "pwrCmd") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::wheelToRoadTorque"))) (name "wheelToRoadTorque") (declared-name "wheelToRoadTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                  )
                )
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature"))) (name "SenseTemperature") (declared-name "SenseTemperature")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature::temp"))) (name "temp") (declared-name "temp") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature")))))
                  )
                )
                (element (kind "action def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque"))) (name "TransferTorque") (declared-name "TransferTorque")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AllocationDefinitions"))) (name "AllocationDefinitions") (declared-name "AllocationDefinitions")
              (contains
                (element (kind "allocation def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AllocationDefinitions::LogicalToPhysical"))) (name "LogicalToPhysical") (declared-name "LogicalToPhysical"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions"))) (name "AttributeDefinitions") (declared-name "AttributeDefinitions")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::*"))) (name "*") (declared-name "*"))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::*#import"))) (name "*") (declared-name "*"))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::*#import2"))) (name "*") (declared-name "*"))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::*#import3"))) (name "*") (declared-name "*"))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::*#import4"))) (name "*") (declared-name "*"))
                (element (kind "enum def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors"))) (name "Colors") (declared-name "Colors")
                  (contains
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors::black"))) (name "black") (declared-name "black") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors")))))
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors::grey"))) (name "grey") (declared-name "grey") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors")))))
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors::red"))) (name "red") (declared-name "red") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors")))))
                  )
                )
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DerivedUnit"))) (name "DerivedUnit") (declared-name "DerivedUnit"))
                (element (kind "enum def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices"))) (name "DiameterChoices") (declared-name "DiameterChoices"))
                (element (kind "enum def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind"))) (name "FuelKind") (declared-name "FuelKind")
                  (contains
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind::diesel"))) (name "diesel") (declared-name "diesel") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind")))))
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind::gas"))) (name "gas") (declared-name "gas") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind")))))
                  )
                )
                (element (kind "enum def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff"))) (name "IgnitionOnOff") (declared-name "IgnitionOnOff")
                  (contains
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff")))))
                    (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff")))))
                  )
                )
                (element (kind "alias") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Torque"))) (name "Torque") (declared-name "Torque"))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (name "cylinderDiameter") (declared-name "cylinderDiameter") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 80)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (role feature-value))))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::kilo"))) (name "kilo") (declared-name "kilo"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext"))) (name "GenericContext") (declared-name "GenericContext")
              (contains
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))) (name "Context") (declared-name "Context") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::accelarationCF"))) (name "accelarationCF") (declared-name "accelarationCF") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "velocityCF")) (expression (kind "featureReference") (reference "s")))))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::accelarationCF"))) (role feature-value))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::spatialCF"))) (name "spatialCF") (declared-name "spatialCF") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::time"))) (name "time") (declared-name "time") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::velocityCF"))) (name "velocityCF") (declared-name "velocityCF") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "spatialCF")) (expression (kind "featureReference") (reference "s")))))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::velocityCF"))) (role feature-value))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions"))) (name "IndividualDefinitions") (declared-name "IndividualDefinitions")
              (contains
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxleAssembly_1"))) (name "FrontAxleAssembly_1") (declared-name "FrontAxleAssembly_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxle_1"))) (name "FrontAxle_1") (declared-name "FrontAxle_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::RearAxleAssembly_1"))) (name "RearAxleAssembly_1") (declared-name "RearAxleAssembly_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Road_1"))) (name "Road_1") (declared-name "Road_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::VehicleRoadContext_1"))) (name "VehicleRoadContext_1") (declared-name "VehicleRoadContext_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Vehicle_1"))) (name "Vehicle_1") (declared-name "Vehicle_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_1"))) (name "Wheel_1") (declared-name "Wheel_1"))
                (element (kind "individual def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_2"))) (name "Wheel_2") (declared-name "Wheel_2"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions"))) (name "InterfaceDefinitions") (declared-name "InterfaceDefinitions")
              (contains
                (element (kind "interface def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface"))) (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface")
                  (contains
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface::p1"))) (name "p1") (declared-name "p1") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface")))))
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface::p2"))) (name "p2") (declared-name "p2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface")))))
                  )
                )
                (element (kind "interface def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface"))) (name "FuelInterface") (declared-name "FuelInterface")
                  (contains
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface")))))
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface::fuelOutPort"))) (name "fuelOutPort") (declared-name "fuelOutPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface")))))
                  )
                )
                (element (kind "interface def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface"))) (name "WheelFastenerInterface") (declared-name "WheelFastenerInterface")
                  (contains
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::lugNutPort"))) (name "lugNutPort") (declared-name "lugNutPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::maxTorque"))) (name "maxTorque") (declared-name "maxTorque") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface")))))
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::shankPort"))) (name "shankPort") (declared-name "shankPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface")))))
                  )
                )
                (element (kind "interface def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface"))) (name "WheelHubInterface") (declared-name "WheelHubInterface")
                  (contains
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface")))))
                    (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions"))) (name "ItemDefinitions") (declared-name "ItemDefinitions")
              (contains
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel"))) (name "Fuel") (declared-name "Fuel")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel::fuelMass"))) (name "fuelMass") (declared-name "fuelMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel")))))
                  )
                )
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (name "FuelCmd") (declared-name "FuelCmd"))
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))) (name "PwrCmd") (declared-name "PwrCmd")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd::throttleLevel"))) (name "throttleLevel") (declared-name "throttleLevel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd")))))
                  )
                )
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed"))) (name "SensedSpeed") (declared-name "SensedSpeed")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed::speed"))) (name "speed") (declared-name "speed") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions"))) (name "KeyWord_MetadataDefinitions") (declared-name "KeyWord_MetadataDefinitions")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
                (element (kind "metadata def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::failureMode"))) (name "failureMode") (declared-name "failureMode")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::failureMode::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::failureMode")))))
                  )
                )
                (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::failureModes"))) (name "failureModes") (declared-name "failureModes") (declared (properties (composite true) (reference false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))))
                (element (kind "metadata def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::logical"))) (name "logical") (declared-name "logical")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::logical::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::logical")))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::logicalOccurrences"))) (name "logicalOccurrences") (declared-name "logicalOccurrences") (declared (properties (composite true) (reference false))))
                (element (kind "metadata def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::physical"))) (name "physical") (declared-name "physical")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::physical::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::physical")))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions::physicalOccurrences"))) (name "physicalOccurrences") (declared-name "physicalOccurrences") (declared (properties (composite true) (reference false))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions"))) (name "MetadataDefinitions") (declared-name "MetadataDefinitions"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions#package"))) (name "MetadataDefinitions") (declared-name "MetadataDefinitions")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions#package::*"))) (name "*") (declared-name "*"))
                (element (kind "metadata def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions#package::Safety"))) (name "Safety") (declared-name "Safety")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions#package::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions#package::Safety")))))
                  )
                )
                (element (kind "metadata def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions#package::Security"))) (name "Security") (declared-name "Security"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions"))) (name "PartDefinitions") (declared-name "PartDefinitions")
              (contains
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))) (name "Axle") (declared-name "Axle") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body"))) (name "Body") (declared-name "Body") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (name "color") (declared-name "color") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))) (name "BodyAssy") (declared-name "BodyAssy") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem"))) (name "BrakingSubsystem") (declared-name "BrakingSubsystem") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (name "CruiseController") (declared-name "CruiseController") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (name "cruiseControlPort") (declared-name "cruiseControlPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController")))))
                    (element (kind "exhibit state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControllerStates"))) (name "cruiseControllerStates") (declared-name "cruiseControllerStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (name "speedSensorPort") (declared-name "speedSensorPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential"))) (name "Differential") (declared-name "Differential") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft"))) (name "Driveshaft") (declared-name "Driveshaft") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator"))) (name "ElectricalGenerator") (declared-name "ElectricalGenerator") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))) (name "Engine") (declared-name "Engine") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::cost"))) (name "cost") (declared-name "cost") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::displacement"))) (name "displacement") (declared-name "displacement") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (name "engineControlPort") (declared-name "engineControlPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "exhibit state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineStates"))) (name "engineStates") (declared-name "engineStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
                      (contains
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
                          (contains
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineStates::on::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                          )
                        )
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                      )
                    )
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::flyWheelPort"))) (name "flyWheelPort") (declared-name "flyWheelPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelEfficiency"))) (name "fuelEfficiency") (declared-name "fuelEfficiency") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (name "ignitionCmdPort") (declared-name "ignitionCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::peakHorsePower"))) (name "peakHorsePower") (declared-name "peakHorsePower") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine4Cyl"))) (name "Engine4Cyl") (declared-name "Engine4Cyl") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine6Cyl"))) (name "Engine6Cyl") (declared-name "Engine6Cyl") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (name "FrontAxle") (declared-name "FrontAxle") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle::steeringAngle"))) (name "steeringAngle") (declared-name "steeringAngle") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (name "FuelTank") (declared-name "FuelTank") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (name "fuelKind") (declared-name "fuelKind") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax"))) (name "fuelMassMax") (declared-name "fuelMassMax") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (name "fuelOutPort") (declared-name "fuelOutPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
                    (element (kind "opaque member") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::item"))) (name "item") (declared-name "item") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::item::fuelMass"))) (name "fuelMass") (declared-name "fuelMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
                      )
                    )
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle"))) (name "HalfAxle") (declared-name "HalfAxle") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (name "Hub") (declared-name "Hub") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road"))) (name "Road") (declared-name "Road") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::friction"))) (name "friction") (declared-name "friction") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::incline"))) (name "incline") (declared-name "incline") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))) (name "Software") (declared-name "Software") (declared (properties (abstract true))))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor"))) (name "SpeedSensor") (declared-name "SpeedSensor") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (name "speedSensorPort") (declared-name "speedSensorPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor"))) (name "StarterMotor") (declared-name "StarterMotor") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (name "gearPort") (declared-name "gearPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem"))) (name "SteeringSubsystem") (declared-name "SteeringSubsystem") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Sunroof"))) (name "Sunroof") (declared-name "Sunroof") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Thermostat"))) (name "Thermostat") (declared-name "Thermostat") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator"))) (name "TorqueGenerator") (declared-name "TorqueGenerator") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::gearRatio"))) (name "gearRatio") (declared-name "gearRatio") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
                    (element (kind "exhibit state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::transmissionStates"))) (name "transmissionStates") (declared-name "transmissionStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionAutomatic"))) (name "TransmissionAutomatic") (declared-name "TransmissionAutomatic") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionChoices"))) (name "TransmissionChoices") (declared-name "TransmissionChoices") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionManual"))) (name "TransmissionManual") (declared-name "TransmissionManual") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::Tmax"))) (name "Tmax") (declared-name "Tmax") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration"))) (name "acceleration") (declared-name "acceleration") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::applyParkingBrake"))) (name "applyParkingBrake") (declared-name "applyParkingBrake") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::brakePedalDepressed"))) (name "brakePedalDepressed") (declared-name "brakePedalDepressed") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))) (name "cargoMass") (declared-name "cargoMass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::controlDirection"))) (name "controlDirection") (declared-name "controlDirection") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))) (name "dryMass") (declared-name "dryMass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::electricalPower"))) (name "electricalPower") (declared-name "electricalPower") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (name "ignitionCmdPort") (declared-name "ignitionCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::maintenanceTime"))) (name "maintenanceTime") (declared-name "maintenanceTime") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::performSelfTest"))) (name "performSelfTest") (declared-name "performSelfTest") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position"))) (name "position") (declared-name "position") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::provideBraking"))) (name "provideBraking") (declared-name "provideBraking") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::providePower"))) (name "providePower") (declared-name "providePower") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (name "pwrCmdPort") (declared-name "pwrCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::senseTemperature"))) (name "senseTemperature") (declared-name "senseTemperature") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (name "statusPort") (declared-name "statusPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "exhibit state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                      (contains
                        (element (kind "ref") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::controller"))) (name "controller") (declared-name "controller") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates"))) (name "healthStates") (declared-name "healthStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::_do::temp"))) (name "temp") (declared-name "temp") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::degraded"))) (name "degraded") (declared-name "degraded") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::degraded_To_normal"))) (name "degraded_To_normal") (declared-name "degraded_To_normal") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::degraded_To_normal::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::maintenance"))) (name "maintenance") (declared-name "maintenance") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::maintenance_To_normal"))) (name "maintenance_To_normal") (declared-name "maintenance_To_normal") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::maintenance_To_normal::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal"))) (name "normal") (declared-name "normal") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal_To_degraded"))) (name "normal_To_degraded") (declared-name "normal_To_degraded") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition effect") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal_To_degraded::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal_To_degraded::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal_To_maintenance"))) (name "normal_To_maintenance") (declared-name "normal_To_maintenance") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal_To_maintenance::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                          )
                        )
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates"))) (name "operatingStates") (declared-name "operatingStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off_To_starting"))) (name "off_To_starting") (declared-name "off_To_starting") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition effect") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off_To_starting::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "transition guard") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off_To_starting::guard"))) (name "guard") (declared-name "guard") (declared (own-expression (expression (kind "binary") (operator "&&") (children (expression (kind "binary") (operator "==") (children (expression (kind "memberAccess") (reference "ignitionOnOff") (children (expression (kind "featureReference") (reference "ignitionCmd")))) (expression (kind "featureReference") (reference "IgnitionOnOff::on")))) (expression (kind "featureReference") (reference "brakePedalDepressed")))))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off_To_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on::_exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on_To_off"))) (name "on_To_off") (declared-name "on_To_off") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition effect") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on_To_off::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on_To_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::starting_To_on"))) (name "starting_To_on") (declared-name "starting_To_on") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::starting_To_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                          )
                        )
                      )
                    )
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity"))) (name "velocity") (declared-name "velocity") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (name "VehicleController") (declared-name "VehicleController") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (name "controlPort") (declared-name "controlPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                    (element (kind "exhibit state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates"))) (name "controllerStates") (declared-name "controllerStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))))
                      (contains
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates"))) (name "operatingStates") (declared-name "operatingStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))))
                          (contains
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::off-on"))) (name "off-on") (declared-name "off-on") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))))
                              (contains
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::off-on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                              )
                            )
                            (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                            (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::on-off"))) (name "on-off") (declared-name "on-off") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))))
                              (contains
                                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::on-off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                              )
                            )
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (name "VehicleSoftware") (declared-name "VehicleSoftware") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::WaterHose"))) (name "WaterHose") (declared-name "WaterHose") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions"))) (name "PortDefinitions") (declared-name "PortDefinitions")
              (contains
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))) (name "AxlePort") (declared-name "AxlePort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort::~AxlePort"))) (name "~AxlePort") (declared-name "~AxlePort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxleToWheelPort"))) (name "AxleToWheelPort") (declared-name "AxleToWheelPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxleToWheelPort::~AxleToWheelPort"))) (name "~AxleToWheelPort") (declared-name "~AxleToWheelPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxleToWheelPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))) (name "ControlPort") (declared-name "ControlPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort::~ControlPort"))) (name "~ControlPort") (declared-name "~ControlPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (name "CruiseControlPort") (declared-name "CruiseControlPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort::~CruiseControlPort"))) (name "~CruiseControlPort") (declared-name "~CruiseControlPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))) (name "DiffPort") (declared-name "DiffPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort::~DiffPort"))) (name "~DiffPort") (declared-name "~DiffPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))) (name "DrivePwrPort") (declared-name "DrivePwrPort")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort::torque"))) (name "torque") (declared-name "torque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort::~DrivePwrPort"))) (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort"))) (name "DriverCmdPort") (declared-name "DriverCmdPort")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort::driverCmd"))) (name "driverCmd") (declared-name "driverCmd") (declared (properties (direction "out") (composite true) (reference false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort::~DriverCmdPort"))) (name "~DriverCmdPort") (declared-name "~DriverCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (name "FuelCmdPort") (declared-name "FuelCmdPort")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort::~FuelCmdPort"))) (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))) (name "FuelPort") (declared-name "FuelPort")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::fuel"))) (name "fuel") (declared-name "fuel") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::~FuelPort"))) (name "~FuelPort") (declared-name "~FuelPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort"))) (name "GearPort") (declared-name "GearPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort::~GearPort"))) (name "~GearPort") (declared-name "~GearPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (name "HandPort") (declared-name "HandPort")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort::ignitionCmd"))) (name "ignitionCmd") (declared-name "ignitionCmd") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort")))))
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort::pwrCmd"))) (name "pwrCmd") (declared-name "pwrCmd") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort::~HandPort"))) (name "~HandPort") (declared-name "~HandPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))) (name "IgnitionCmdPort") (declared-name "IgnitionCmdPort")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort::ignitionCmd"))) (name "ignitionCmd") (declared-name "ignitionCmd") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort::~IgnitionCmdPort"))) (name "~IgnitionCmdPort") (declared-name "~IgnitionCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))) (name "LugNutCompositePort") (declared-name "LugNutCompositePort")
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (name "lugNutPort") (declared-name "lugNutPort") (declared (properties (composite true) (reference false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::~LugNutCompositePort"))) (name "~LugNutCompositePort") (declared-name "~LugNutCompositePort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))) (name "LugNutPort") (declared-name "LugNutPort")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadDia"))) (name "threadDia") (declared-name "threadDia") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadPitch"))) (name "threadPitch") (declared-name "threadPitch") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::~LugNutPort"))) (name "~LugNutPort") (declared-name "~LugNutPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))) (name "PwrCmdPort") (declared-name "PwrCmdPort")
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort::pwrCmd"))) (name "pwrCmd") (declared-name "pwrCmd") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort::~PwrCmdPort"))) (name "~PwrCmdPort") (declared-name "~PwrCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort"))) (name "SetSpeedPort") (declared-name "SetSpeedPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort::~SetSpeedPort"))) (name "~SetSpeedPort") (declared-name "~SetSpeedPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a"))) (name "ShaftPort_a") (declared-name "ShaftPort_a")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a::~ShaftPort_a"))) (name "~ShaftPort_a") (declared-name "~ShaftPort_a") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b"))) (name "ShaftPort_b") (declared-name "ShaftPort_b")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b::~ShaftPort_b"))) (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c"))) (name "ShaftPort_c") (declared-name "ShaftPort_c")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c::~ShaftPort_c"))) (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))) (name "ShaftPort_d") (declared-name "ShaftPort_d")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d::~ShaftPort_d"))) (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))) (name "ShankCompositePort") (declared-name "ShankCompositePort")
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (name "shankPort") (declared-name "shankPort") (declared (properties (composite true) (reference false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::~ShankCompositePort"))) (name "~ShankCompositePort") (declared-name "~ShankCompositePort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))) (name "ShankPort") (declared-name "ShankPort")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::shaftLength"))) (name "shaftLength") (declared-name "shaftLength") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadDia"))) (name "threadDia") (declared-name "threadDia") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadPitch"))) (name "threadPitch") (declared-name "threadPitch") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::~ShankPort"))) (name "~ShankPort") (declared-name "~ShankPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort"))) (name "SpeedSensorPort") (declared-name "SpeedSensorPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort::~SpeedSensorPort"))) (name "~SpeedSensorPort") (declared-name "~SpeedSensorPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort"))) (name "StatusPort") (declared-name "StatusPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort::~StatusPort"))) (name "~StatusPort") (declared-name "~StatusPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort"))) (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort::~VehicleToRoadPort"))) (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToAxlePort"))) (name "WheelToAxlePort") (declared-name "WheelToAxlePort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToAxlePort::~WheelToAxlePort"))) (name "~WheelToAxlePort") (declared-name "~WheelToAxlePort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToAxlePort")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))) (name "WheelToRoadPort") (declared-name "WheelToRoadPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort::~WheelToRoadPort"))) (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions"))) (name "RequirementDefinitions") (declared-name "RequirementDefinitions")
              (contains
                (element (kind "requirement def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement"))) (name "DrivePowerOutputRequirement") (declared-name "DrivePowerOutputRequirement")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement")))))
                  )
                )
                (element (kind "requirement def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))) (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                    (element (kind "require constraint") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::actualFuelEconomy"))) (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                  )
                )
                (element (kind "requirement def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))) (name "MassRequirement") (declared-name "MassRequirement")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                    (element (kind "require constraint") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired"))) (name "massRequired") (declared-name "massRequired") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                  )
                )
                (element (kind "requirement def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement"))) (name "ReliabilityRequirement") (declared-name "ReliabilityRequirement")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement")))))
                    (element (kind "require constraint") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityActual"))) (name "reliabilityActual") (declared-name "reliabilityActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityRequired"))) (name "reliabilityRequired") (declared-name "reliabilityRequired") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement")))))
                  )
                )
                (element (kind "requirement def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement"))) (name "TorqueGenerationRequirement") (declared-name "TorqueGenerationRequirement")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement")))))
                    (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions"))) (name "SignalDefinitions") (declared-name "SignalDefinitions")
              (contains
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::Cmd"))) (name "Cmd") (declared-name "Cmd"))
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd"))) (name "DriverCmd") (declared-name "DriverCmd"))
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::EngineStatus"))) (name "EngineStatus") (declared-name "EngineStatus"))
                (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (name "IgnitionCmd") (declared-name "IgnitionCmd")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (name "ignitionOnOff") (declared-name "ignitionOnOff") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd")))))
                  )
                )
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::OffSignal"))) (name "OffSignal") (declared-name "OffSignal") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::OverTemp"))) (name "OverTemp") (declared-name "OverTemp") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::ReturnToNormal"))) (name "ReturnToNormal") (declared-name "ReturnToNormal") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::SetSpeed"))) (name "SetSpeed") (declared-name "SetSpeed") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::StartSignal"))) (name "StartSignal") (declared-name "StartSignal") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::VehicleOffSignal"))) (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::VehicleOnSignal"))) (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (declared (properties (ordered false) (unique true))))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::VehicleStartSignal"))) (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (declared (properties (ordered false) (unique true))))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions"))) (name "StateDefinitions") (declared-name "StateDefinitions")
              (contains
                (element (kind "state def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions::ControllerStates"))) (name "ControllerStates") (declared-name "ControllerStates"))
                (element (kind "state def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions::CruiseControllerStates"))) (name "CruiseControllerStates") (declared-name "CruiseControllerStates"))
                (element (kind "state def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions::VehicleStates"))) (name "VehicleStates") (declared-name "VehicleStates"))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext"))) (name "MissionContext") (declared-name "MissionContext")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::*"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions"))) (name "ContextDefinitions") (declared-name "ContextDefinitions")
              (contains
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))) (name "Driver") (declared-name "Driver") (declared)
                  (contains
                    (element (kind "exhibit state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates"))) (name "driverStates") (declared-name "driverStates") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))))
                      (contains
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                        (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::initial#transition"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait"))) (name "wait") (declared-name "wait") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                        (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait-wait-1"))) (name "wait-wait-1") (declared-name "wait-wait-1") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))))
                          (contains
                            (element (kind "transition effect") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait-wait-1::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                          )
                        )
                        (element (kind "transition") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait-wait-2"))) (name "wait-wait-2") (declared-name "wait-wait-2") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))))
                          (contains
                            (element (kind "transition effect") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait-wait-2::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                          )
                        )
                      )
                    )
                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (name "handPort") (declared-name "handPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                  )
                )
                (element (kind "use case def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle"))) (name "GetInVehicle") (declared-name "GetInVehicle")
                  (contains
                    (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle")))))
                  )
                )
                (element (kind "use case def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle"))) (name "GetOutOfVehicle") (declared-name "GetOutOfVehicle")
                  (contains
                    (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (name "MissionContext") (declared-name "MissionContext") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger"))) (name "Passenger") (declared-name "Passenger") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Road"))) (name "Road") (declared-name "Road") (declared))
                (element (kind "use case def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger"))) (name "TransportPassenger") (declared-name "TransportPassenger")
                  (contains
                    (element (kind "objective") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::TransportObjective"))) (name "TransportObjective") (declared-name "TransportObjective") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                  )
                )
                (element (kind "use case") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle"))) (name "getInVehicle") (declared-name "getInVehicle")
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::closeDoor_in"))) (name "closeDoor_in") (declared-name "closeDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::enterVehicle"))) (name "enterVehicle") (declared-name "enterVehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::openDoor_in"))) (name "openDoor_in") (declared-name "openDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::unlockDoor_in"))) (name "unlockDoor_in") (declared-name "unlockDoor_in") (declared (properties (composite true) (reference false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle")))))
                  )
                )
                (element (kind "use case") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle"))) (name "getOutOfVehicle") (declared-name "getOutOfVehicle")
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::closeDoor_out"))) (name "closeDoor_out") (declared-name "closeDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::exitVehicle"))) (name "exitVehicle") (declared-name "exitVehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::lockDoor_out"))) (name "lockDoor_out") (declared-name "lockDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::openDoor_out"))) (name "openDoor_out") (declared-name "openDoor_out") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle")))))
                  )
                )
                (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::transportRequirements"))) (name "transportRequirements") (declared-name "transportRequirements"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario"))) (name "TransportPassengerScenario") (declared-name "TransportPassengerScenario")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::TransportPassenger"))) (name "TransportPassenger") (declared-name "TransportPassenger"))
                (element (kind "use case") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (name "transportPassenger") (declared-name "transportPassenger")
                  (contains
                    (element (kind "verdict") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::_verdict"))) (name "done") (declared-name "done") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::b"))) (name "b") (declared-name "b") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::c"))) (name "c") (declared-name "c") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "succession") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::start"))) (name "start") (declared-name "start") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                  )
                )
                (element (kind "use case") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1"))) (name "transportPassenger_1") (declared-name "transportPassenger_1")
                  (contains
                    (element (kind "verdict") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::_verdict"))) (name "done") (declared-name "done") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driveVehicleToDestination"))) (name "driveVehicleToDestination") (declared-name "driveVehicleToDestination") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driverGetInVehicle"))) (name "driverGetInVehicle") (declared-name "driverGetInVehicle") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driverGetOutOfVehicle"))) (name "driverGetOutOfVehicle") (declared-name "driverGetOutOfVehicle") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::passenger1GetInVehicle"))) (name "passenger1GetInVehicle") (declared-name "passenger1GetInVehicle") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::passenger1GetOutOfVehicle"))) (name "passenger1GetOutOfVehicle") (declared-name "passenger1GetOutOfVehicle") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::providePower"))) (name "providePower") (declared-name "providePower") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "succession") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::start"))) (name "start") (declared-name "start") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::trigger"))) (name "trigger") (declared-name "trigger") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger")))))
                  )
                )
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::VehicleConfiguration_b"))) (name "VehicleConfiguration_b") (declared-name "VehicleConfiguration_b"))
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (name "missionContext") (declared-name "missionContext") (declared (properties (composite true) (reference false) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (name "driver") (declared-name "driver") (declared (properties (composite true) (reference false) (ordered false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "driver") (children (expression (kind "featureReference") (reference "transportPassenger")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (role feature-value)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.a.driverGetInVehicle.closeDoor_in"))) (name "transportPassenger.a.driverGetInVehicle.closeDoor_in") (declared-name "transportPassenger.a.driverGetInVehicle.closeDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.a.driverGetInVehicle.enterVehicle"))) (name "transportPassenger.a.driverGetInVehicle.enterVehicle") (declared-name "transportPassenger.a.driverGetInVehicle.enterVehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.a.driverGetInVehicle.openDoor_in"))) (name "transportPassenger.a.driverGetInVehicle.openDoor_in") (declared-name "transportPassenger.a.driverGetInVehicle.openDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.a.driverGetInVehicle.unlockDoor_in"))) (name "transportPassenger.a.driverGetInVehicle.unlockDoor_in") (declared-name "transportPassenger.a.driverGetInVehicle.unlockDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.b.driveVehicleToDestination"))) (name "transportPassenger.b.driveVehicleToDestination") (declared-name "transportPassenger.b.driveVehicleToDestination") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.c.driverGetOutOfVehicle.closeDoor_out"))) (name "transportPassenger.c.driverGetOutOfVehicle.closeDoor_out") (declared-name "transportPassenger.c.driverGetOutOfVehicle.closeDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.c.driverGetOutOfVehicle.exitVehicle"))) (name "transportPassenger.c.driverGetOutOfVehicle.exitVehicle") (declared-name "transportPassenger.c.driverGetOutOfVehicle.exitVehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.c.driverGetOutOfVehicle.lockDoor_out"))) (name "transportPassenger.c.driverGetOutOfVehicle.lockDoor_out") (declared-name "transportPassenger.c.driverGetOutOfVehicle.lockDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger.c.driverGetOutOfVehicle.openDoor_out"))) (name "transportPassenger.c.driverGetOutOfVehicle.openDoor_out") (declared-name "transportPassenger.c.driverGetOutOfVehicle.openDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (name "passenger1") (declared-name "passenger1") (declared (properties (composite true) (reference false) (ordered false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "passenger") (children (expression (kind "featureReference") (reference "transportPassenger")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (role feature-value)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.a.passenger1GetInVehicle.closeDoor_in"))) (name "transportPassenger.a.passenger1GetInVehicle.closeDoor_in") (declared-name "transportPassenger.a.passenger1GetInVehicle.closeDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.a.passenger1GetInVehicle.enterVehicle"))) (name "transportPassenger.a.passenger1GetInVehicle.enterVehicle") (declared-name "transportPassenger.a.passenger1GetInVehicle.enterVehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.a.passenger1GetInVehicle.openDoor_in"))) (name "transportPassenger.a.passenger1GetInVehicle.openDoor_in") (declared-name "transportPassenger.a.passenger1GetInVehicle.openDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.a.passenger1GetInVehicle.unlockDoor_in"))) (name "transportPassenger.a.passenger1GetInVehicle.unlockDoor_in") (declared-name "transportPassenger.a.passenger1GetInVehicle.unlockDoor_in") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out"))) (name "transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out") (declared-name "transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle"))) (name "transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle") (declared-name "transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out"))) (name "transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out") (declared-name "transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out"))) (name "transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out") (declared-name "transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (name "road") (declared-name "road") (declared (properties (composite true) (reference false) (ordered false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "road") (children (expression (kind "featureReference") (reference "transportPassenger")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (role feature-value))))
                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::transportPassenger"))) (name "transportPassenger") (declared-name "transportPassenger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1"))) (name "vehicle_b_1") (declared-name "vehicle_b_1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1::position3dVector"))) (name "position3dVector") (declared-name "position3dVector") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower unevaluated) (upper unevaluated) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "tuple") (children (expression (kind "integerLiteral") (literal 0)) (expression (kind "integerLiteral") (literal 0)) (expression (kind "integerLiteral") (literal 0)))))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1::position3dVector"))) (role feature-value))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1::transportPassenger.trigger"))) (name "transportPassenger.trigger") (declared-name "transportPassenger.trigger") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext")))))
                  )
                )
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::moe"))) (name "moe") (declared-name "moe"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups"))) (name "SafetyandSecurityGroups") (declared-name "SafetyandSecurityGroups")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::*"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::MandatorySafetyGroup"))) (name "MandatorySafetyGroup") (declared-name "MandatorySafetyGroup")
              (contains
                (element (kind "filter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::MandatorySafetyGroup::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "&&") (children (expression (kind "classification") (reference "Safety")) (expression (kind "featureReference") (reference "Safety::isMandatory")))))))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::MandatorySafetyGroup::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyGroup"))) (name "SafetyGroup") (declared-name "SafetyGroup")
              (contains
                (element (kind "filter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyGroup::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "Safety")))))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyGroup::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyandSecurityGroup"))) (name "SafetyandSecurityGroup") (declared-name "SafetyandSecurityGroup")
              (contains
                (element (kind "filter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyandSecurityGroup::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "||") (children (expression (kind "classification") (reference "Safety")) (expression (kind "classification") (reference "Security")))))))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyandSecurityGroup::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SecurityGroup"))) (name "SecurityGroup") (declared-name "SecurityGroup")
              (contains
                (element (kind "filter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SecurityGroup::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "Security")))))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SecurityGroup::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b"))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis"))) (name "VehicleAnalysis") (declared-name "VehicleAnalysis")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::*#import"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::ElectricalPowerAnalysis"))) (name "ElectricalPowerAnalysis") (declared-name "ElectricalPowerAnalysis"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel"))) (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel")
              (contains
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::AverageTravelTimePerDistance"))) (name "AverageTravelTimePerDistance") (declared-name "AverageTravelTimePerDistance")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::AverageTravelTimePerDistance::scenario"))) (name "scenario") (declared-name "scenario") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::AverageTravelTimePerDistance")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::AverageTravelTimePerDistance::tpd_avg"))) (name "tpd_avg") (declared-name "tpd_avg") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::AverageTravelTimePerDistance")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance"))) (name "BestFuelConsumptionPerDistance") (declared-name "BestFuelConsumptionPerDistance")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance::bsfc"))) (name "bsfc") (declared-name "bsfc") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance::distance"))) (name "distance") (declared-name "distance") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance::f_b"))) (name "f_b") (declared-name "f_b") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance::tpd_avg"))) (name "tpd_avg") (declared-name "tpd_avg") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::BestFuelConsumptionPerDistance")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::ComputeBSFC"))) (name "ComputeBSFC") (declared-name "ComputeBSFC")
                  (contains
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::ComputeBSFC::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::ComputeBSFC")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::ComputeBSFC::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::ComputeBSFC")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption"))) (name "FuelConsumption") (declared-name "FuelConsumption")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption::bestFuelConsumption"))) (name "bestFuelConsumption") (declared-name "bestFuelConsumption") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption::dpv"))) (name "dpv") (declared-name "dpv") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption::idlingFuelConsumption"))) (name "idlingFuelConsumption") (declared-name "idlingFuelConsumption") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption::tpd_avg"))) (name "tpd_avg") (declared-name "tpd_avg") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::FuelConsumption")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::IdlingFuelConsumptionPerTime"))) (name "IdlingFuelConsumptionPerTime") (declared-name "IdlingFuelConsumptionPerTime")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::IdlingFuelConsumptionPerTime::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::IdlingFuelConsumptionPerTime")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::IdlingFuelConsumptionPerTime::f_a"))) (name "f_a") (declared-name "f_a") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::IdlingFuelConsumptionPerTime")))))
                  )
                )
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::SampledFunction"))) (name "SampledFunction") (declared-name "SampledFunction"))
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))) (name "Scenario") (declared-name "Scenario") (declared (properties (ordered false) (unique true)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint"))) (name "wayPoint") (declared-name "wayPoint") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::TraveledDistance"))) (name "TraveledDistance") (declared-name "TraveledDistance")
                  (contains
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::TraveledDistance::distance"))) (name "distance") (declared-name "distance") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::TraveledDistance")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::TraveledDistance::scenario"))) (name "scenario") (declared-name "scenario") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::TraveledDistance")))))
                  )
                )
                (element (kind "analysis") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis"))) (name "fuelEconomyAnalysis") (declared-name "fuelEconomyAnalysis")
                  (contains
                    (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::"))) (name ""))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::bsfc"))) (name "bsfc") (declared-name "bsfc") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "ComputeBSFC"))) (arguments (argument (expression (kind "memberAccess") (reference "engine") (children (expression (kind "featureReference") (reference "vehicle_b"))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::bsfc"))) (role feature-value))))
                    (element (kind "analysis result") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::calculatedFuelEconomy"))) (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy"))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::distance"))) (name "distance") (declared-name "distance") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "TraveledDistance"))) (arguments (argument (expression (kind "featureReference") (reference "scenario"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::distance"))) (role feature-value))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::f_a"))) (name "f_a") (declared-name "f_a") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "BestFuelConsumptionPerDistance"))) (arguments (argument (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "vehicle_b"))))) (argument (expression (kind "featureReference") (reference "bsfc"))) (argument (expression (kind "featureReference") (reference "tpd_avg"))) (argument (expression (kind "featureReference") (reference "distance"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::f_a"))) (role feature-value))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::f_b"))) (name "f_b") (declared-name "f_b") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "IdlingFuelConsumptionPerTime"))) (arguments (argument (expression (kind "memberAccess") (reference "engine") (children (expression (kind "featureReference") (reference "vehicle_b"))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::f_b"))) (role feature-value))))
                    (element (kind "objective") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective"))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::scenario"))) (name "scenario") (declared-name "scenario") (declared (properties (direction "in") (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::tpd_avg"))) (name "tpd_avg") (declared-name "tpd_avg") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "AverageTravelTimePerDistance"))) (arguments (argument (expression (kind "featureReference") (reference "scenario"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::tpd_avg"))) (role feature-value))))
                  )
                )
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::specificGravityOfGasoline"))) (name "specificGravityOfGasoline") (declared-name "specificGravityOfGasoline") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "realLiteral") (literal "0.76")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::specificGravityOfGasoline"))) (role feature-value))))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::ReliabilityAnalyis"))) (name "ReliabilityAnalyis") (declared-name "ReliabilityAnalyis"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleConfiguration_b"))) (name "VehicleConfiguration_b") (declared-name "VehicleConfiguration_b"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis"))) (name "VehicleTradeOffAnalysis") (declared-name "VehicleTradeOffAnalysis")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::*"))) (name "*") (declared-name "*"))
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation"))) (name "EngineEvaluation") (declared-name "EngineEvaluation")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation::engineCost"))) (name "engineCost") (declared-name "engineCost") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation::engineFuelEfficiency"))) (name "engineFuelEfficiency") (declared-name "engineFuelEfficiency") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation::engineMass"))) (name "engineMass") (declared-name "engineMass") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation::enginePower"))) (name "enginePower") (declared-name "enginePower") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation::eval"))) (name "eval") (declared-name "eval") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl"))) (name "EngineEvaluation_4cyl") (declared-name "EngineEvaluation_4cyl")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl::engineCost"))) (name "engineCost") (declared-name "engineCost") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl::engineFuelEfficiency"))) (name "engineFuelEfficiency") (declared-name "engineFuelEfficiency") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl::engineMass"))) (name "engineMass") (declared-name "engineMass") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl::enginePower"))) (name "enginePower") (declared-name "enginePower") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl::eval"))) (name "eval") (declared-name "eval") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_4cyl")))))
                  )
                )
                (element (kind "calc def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl"))) (name "EngineEvaluation_6cyl") (declared-name "EngineEvaluation_6cyl")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl::engineCost"))) (name "engineCost") (declared-name "engineCost") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl::engineFuelEfficiency"))) (name "engineFuelEfficiency") (declared-name "engineFuelEfficiency") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl::engineMass"))) (name "engineMass") (declared-name "engineMass") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl::enginePower"))) (name "enginePower") (declared-name "enginePower") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl")))))
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl::eval"))) (name "eval") (declared-name "eval") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::EngineEvaluation_6cyl")))))
                  )
                )
                (element (kind "analysis") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis"))) (name "engineTradeOffAnalysis") (declared-name "engineTradeOffAnalysis")
                  (contains
                    (element (kind "calc") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::"))) (name "")
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (direction "in") (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                    (element (kind "calc") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::#calc"))) (name "")
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle#part"))) (name "vehicle") (declared-name "vehicle") (declared (properties (direction "in") (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                    (element (kind "objective") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::objective"))) (name "objective") (declared-name "objective"))
                    (element (kind "analysis result") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::selectedVehicle"))) (name "selectedVehicle") (declared-name "selectedVehicle"))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl"))) (name "vehicle_b_engine4cyl") (declared-name "vehicle_b_engine4cyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::cost"))) (name "cost") (declared-name "cost") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 1000)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::cost"))) (role feature-value))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::cylinders"))) (name "cylinders") (declared-name "cylinders") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 180)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::mass"))) (role feature-value))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::peakHorsePower"))) (name "peakHorsePower") (declared-name "peakHorsePower") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 180)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "W")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::peakHorsePower"))) (role feature-value))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl"))) (name "vehicle_b_engine6cyl") (declared-name "vehicle_b_engine6cyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::cost"))) (name "cost") (declared-name "cost") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 1500)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::cost"))) (role feature-value))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::cylinders"))) (name "cylinders") (declared-name "cylinders") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 220)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::mass"))) (role feature-value))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::peakHorsePower"))) (name "peakHorsePower") (declared-name "peakHorsePower") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 220)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "W")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl::engine::peakHorsePower"))) (role feature-value))))
                          )
                        )
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations"))) (name "VehicleConfigurations") (declared-name "VehicleConfigurations")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant"))) (name "Engine4Cyl_Variant") (declared-name "Engine4Cyl_Variant")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::*"))) (name "*") (declared-name "*"))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::_refinement"))) (name "refinement") (declared-name "refinement"))
                (element (kind "dependency") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::dependency"))) (name "dependency") (declared-name "dependency"))
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (name "cylinders") (declared-name "cylinders") (declared (properties (composite true) (reference false) (ordered true)) (multiplicity (lower 4) (upper 8) (ordered true) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl"))) (name "engine4Cyl") (declared-name "engine4Cyl") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder1"))) (name "cylinder1") (declared-name "cylinder1") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder2"))) (name "cylinder2") (declared-name "cylinder2") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder3"))) (name "cylinder3") (declared-name "cylinder3") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder4"))) (name "cylinder4") (declared-name "cylinder4") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinders"))) (name "cylinders") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a"))) (name "VehicleConfiguration_a") (declared-name "VehicleConfiguration_a")
              (contains
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::ActionTree"))) (name "ActionTree") (declared-name "ActionTree"))
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (name "vehicle_a") (declared-name "vehicle_a") (declared (properties (composite true) (reference false) (ordered false)))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::cargoMass"))) (name "cargoMass") (declared-name "cargoMass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 0)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::cargoMass"))) (role feature-value))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::dryMass"))) (name "dryMass") (declared-name "dryMass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "featureReference") (reference "partMasses"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::dryMass"))) (role feature-value))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (name "frontWheels") (declared-name "frontWheels") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 800)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::mass"))) (role feature-value))))
                          )
                        )
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (name "fuelTank") (declared-name "fuelTank") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 75)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank::mass"))) (role feature-value))))
                          )
                        )
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "dryMass")) (expression (kind "featureReference") (reference "cargoMass")))) (expression (kind "memberAccess") (reference "fuelMass") (children (expression (kind "memberAccess") (reference "fuel") (children (expression (kind "featureReference") (reference "fuelTank")))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))) (role feature-value))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::partMasses"))) (name "partMasses") (declared-name "partMasses") (declared (properties (composite true) (reference false) (ordered false) (unique false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (unique false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::driveTrainEfficiency"))) (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "realLiteral") (literal "0.6")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::driveTrainEfficiency"))) (role feature-value))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 875)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::mass"))) (role feature-value))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (name "rearWheels") (declared-name "rearWheels") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                              (contains
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                              )
                            )
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::Requirements"))) (name "Requirements") (declared-name "Requirements"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b"))) (name "VehicleConfiguration_b") (declared-name "VehicleConfiguration_b")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::*"))) (name "*") (declared-name "*"))
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree"))) (name "ActionTree") (declared-name "ActionTree")
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (name "applyParkingBrake") (declared-name "applyParkingBrake") (declared (properties (composite true) (reference false))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (name "performSelfTest") (declared-name "performSelfTest") (declared (properties (composite true) (reference false))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (name "providePower") (declared-name "providePower") (declared (properties (composite true) (reference false)))
                      (contains
                        (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (name "amplifyTorque") (declared-name "amplifyTorque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (name "distributeTorque") (declared-name "distributeTorque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                        (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                        (element (kind "flow") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateToAmplify"))) (name "generateToAmplify") (declared-name "generateToAmplify") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower"))))
                          (contains
                            (element (kind "item") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque::"))) (name "") (declared (properties (direction "in") (composite true) (reference false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "fuelCmd") (children (expression (kind "featureReference") (reference "providePower")))))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque::"))) (role feature-value))))
                          )
                        )
                        (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (name "transferTorque") (declared-name "transferTorque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::wheelToRoadTorque"))) (name "wheelToRoadTorque") (declared-name "wheelToRoadTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
                      )
                    )
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (name "senseTemperature") (declared-name "senseTemperature") (declared (properties (composite true) (reference false))))
                  )
                )
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Box"))) (name "Box") (declared-name "Box"))
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions"))) (name "DiscreteInteractions") (declared-name "DiscreteInteractions")
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1"))) (name "CruiseControl1") (declared-name "CruiseControl1") (declared (properties (composite true) (reference false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::engine"))) (name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::engine::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                              )
                            )
                            (element (kind "flow") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::sendFuelCmd"))) (name "sendFuelCmd") (declared-name "sendFuelCmd")
                              (contains
                                (element (kind "flow payload") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::sendFuelCmd::_payload"))) (name "_payload") (declared-name "_payload"))
                              )
                            )
                            (element (kind "flow") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::sendSensedSpeed"))) (name "sendSensedSpeed") (declared-name "sendSensedSpeed")
                              (contains
                                (element (kind "flow payload") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::sendSensedSpeed::_payload"))) (name "_payload") (declared-name "_payload"))
                              )
                            )
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::speedSensor"))) (name "speedSensor") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::speedSensor::speedSensorPort"))) (name "speedSensorPort") (declared-name "speedSensorPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::vehicleSoftware"))) (name "vehicleSoftware") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::vehicleSoftware::vehicleController"))) (name "vehicleController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                                  (contains
                                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (name "cruiseController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                                      (contains
                                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::vehicleSoftware::vehicleController::cruiseController::cruiseControlPort"))) (name "cruiseControlPort") (declared-name "cruiseControlPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::vehicleSoftware::vehicleController::cruiseController::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::vehicleSoftware::vehicleController::cruiseController::speedSensorPort"))) (name "speedSensorPort") (declared-name "speedSensorPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                                      )
                                    )
                                  )
                                )
                              )
                            )
                          )
                        )
                      )
                    )
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2"))) (name "CruiseControl2") (declared-name "CruiseControl2") (declared (properties (composite true) (reference false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::engine"))) (name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::engine::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                              )
                            )
                            (element (kind "flow") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::sendFuelCmd"))) (name "sendFuelCmd") (declared-name "sendFuelCmd")
                              (contains
                                (element (kind "flow payload") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::sendFuelCmd::_payload"))) (name "_payload") (declared-name "_payload"))
                              )
                            )
                            (element (kind "flow") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::sendSensedSpeed"))) (name "sendSensedSpeed") (declared-name "sendSensedSpeed")
                              (contains
                                (element (kind "flow payload") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::sendSensedSpeed::_payload"))) (name "_payload") (declared-name "_payload"))
                              )
                            )
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::speedSensor"))) (name "speedSensor") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::speedSensor::speedSensorPort"))) (name "speedSensorPort") (declared-name "speedSensorPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::vehicleSoftware"))) (name "vehicleSoftware") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::vehicleSoftware::vehicleController"))) (name "vehicleController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                                  (contains
                                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (name "cruiseController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                                      (contains
                                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::vehicleSoftware::vehicleController::cruiseController::cruiseControlPort"))) (name "cruiseControlPort") (declared-name "cruiseControlPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::vehicleSoftware::vehicleController::cruiseController::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::vehicleSoftware::vehicleController::cruiseController::speedSensorPort"))) (name "speedSensorPort") (declared-name "speedSensorPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                                      )
                                    )
                                  )
                                )
                              )
                            )
                          )
                        )
                      )
                    )
                    (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence"))) (name "Sequence") (declared-name "Sequence")
                      (contains
                        (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver"))) (name "Driver") (declared-name "Driver") (declared)
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver::p1"))) (name "p1") (declared-name "p1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver::p2"))) (name "p2") (declared-name "p2") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
                          )
                        )
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0"))) (name "part0") (declared-name "part0") (declared (properties (composite true) (reference false) (ordered false)))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (name "driver") (declared-name "driver") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver::driverReady"))) (name "driverReady") (declared-name "driverReady") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver::startVehicle.trigger2"))) (name "startVehicle.trigger2") (declared-name "startVehicle.trigger2") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver::startVehicle.turnVehicleOn"))) (name "startVehicle.turnVehicleOn") (declared-name "startVehicle.turnVehicleOn") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                              (contains
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle::doorClosed"))) (name "doorClosed") (declared-name "doorClosed") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle::startVehicle.sendStatus"))) (name "startVehicle.sendStatus") (declared-name "startVehicle.sendStatus") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle::startVehicle.trigger1"))) (name "startVehicle.trigger1") (declared-name "startVehicle.trigger1") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b") (declared (properties (composite true) (reference false) (ordered false)))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::avgFuelEconomy"))) (name "avgFuelEconomy") (declared-name "avgFuelEconomy") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (name "bodyAssy") (declared-name "bodyAssy") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (name "body") (declared-name "body") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))))
                              (contains
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body::color"))) (name "color") (declared-name "color") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "Colors::red")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body::color"))) (role feature-value))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::bumper"))) (name "bumper") (declared-name "bumper") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))))
                              (contains
                                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::bumper::Safety"))) (name "Safety") (declared-name "Safety") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))))
                                  (contains
                                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::bumper::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy")))))
                                  )
                                )
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::keylessEntry"))) (name "keylessEntry") (declared-name "keylessEntry") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))))
                              (contains
                                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::keylessEntry::Security"))) (name "Security") (declared-name "Security") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy")))))
                              )
                            )
                          )
                        )
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::cargoMass"))) (name "cargoMass") (declared-name "cargoMass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 0)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (name "driveshaft") (declared-name "driveshaft") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 100)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::mass"))) (role feature-value))))
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::providePower.transferTorque"))) (name "providePower.transferTorque") (declared-name "providePower.transferTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (name "shaftPort_b") (declared-name "shaftPort_b") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (name "shaftPort_c") (declared-name "shaftPort_c") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft")))))
                          )
                        )
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::dryMass"))) (name "dryMass") (declared-name "dryMass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "featureReference") (reference "partMasses"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::dryMass"))) (role feature-value))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::alternator"))) (name "alternator") (declared-name "alternator") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
                              (contains
                                (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::alternator::generateElectricity"))) (name "generateElectricity") (declared-name "generateElectricity") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (name "cylinders") (declared-name "cylinders") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 6) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                          )
                        )
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (name "frontWheels") (declared-name "frontWheels") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 800)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::mass"))) (role feature-value))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                          )
                        )
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (name "fuelTank") (declared-name "fuelTank") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank::fuelMassMax"))) (name "fuelMassMax") (declared-name "fuelMassMax") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 60)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank::fuelMassMax"))) (role feature-value))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 75)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank::mass"))) (role feature-value))))
                          )
                        )
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior"))) (name "interior") (declared-name "interior") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::alarm"))) (name "alarm") (declared-name "alarm") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::alarm::Security"))) (name "Security") (declared-name "Security") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::driverAirBag"))) (name "driverAirBag") (declared-name "driverAirBag") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::driverAirBag::Safety"))) (name "Safety") (declared-name "Safety") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                                  (contains
                                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::driverAirBag::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                  )
                                )
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::frontSeat"))) (name "frontSeat") (declared-name "frontSeat") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::seatBelt"))) (name "seatBelt") (declared-name "seatBelt") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                              (contains
                                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::seatBelt::Safety"))) (name "Safety") (declared-name "Safety") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                                  (contains
                                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::seatBelt::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                                  )
                                )
                              )
                            )
                          )
                        )
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::partMasses"))) (name "partMasses") (declared-name "partMasses") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "tuple") (children (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "fuelTank")))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "frontAxleAssembly")))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "rearAxleAssembly")))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "engine")))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "transmission")))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "driveshaft")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::partMasses"))) (role feature-value))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (name "differential") (declared-name "differential") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                              (contains
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (name "leftDiffPort") (declared-name "leftDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential")))))
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (name "rightDiffPort") (declared-name "rightDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential")))))
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential")))))
                              )
                            )
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::driveTrainEfficiency"))) (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "realLiteral") (literal "0.6")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::driveTrainEfficiency"))) (role feature-value))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 875)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::mass"))) (role feature-value))))
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::providePower.distributeTorque"))) (name "providePower.distributeTorque") (declared-name "providePower.distributeTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                              (contains
                                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (name "leftHalfAxle") (declared-name "leftHalfAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                                  (contains
                                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (name "leftAxleToDiffPort") (declared-name "leftAxleToDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
                                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
                                  )
                                )
                                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (name "rightHalfAxle") (declared-name "rightHalfAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                                  (contains
                                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (name "rightAxleToDiffPort") (declared-name "rightAxleToDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
                                    (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
                                  )
                                )
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (name "rearWheel1") (declared-name "rearWheel1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                              (contains
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                              )
                            )
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (name "rearWheel2") (declared-name "rearWheel2") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
                              (contains
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                                (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                              )
                            )
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
                          )
                        )
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (name "speedSensor") (declared-name "speedSensor") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (name "starterMotor") (declared-name "starterMotor") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 100)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::mass"))) (role feature-value))))
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::providePower.amplifyTorque"))) (name "providePower.amplifyTorque") (declared-name "providePower.amplifyTorque") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (name "shaftPort_a") (declared-name "shaftPort_a") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
                          )
                        )
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (name "vehicleSoftware") (declared-name "vehicleSoftware") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (name "vehicleController") (declared-name "vehicleController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))))
                              (contains
                                (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::controllerStates"))) (name "controllerStates") (declared-name "controllerStates") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (name "cruiseController") (declared-name "cruiseController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
                              )
                            )
                          )
                        )
                        (element (kind "state") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort"))) (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (name "wheelToRoadPort1") (declared-name "wheelToRoadPort1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (name "wheelToRoadPort2") (declared-name "wheelToRoadPort2") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements"))) (name "Requirements") (declared-name "Requirements")
                  (contains
                    (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::*"))) (name "*") (declared-name "*"))
                    (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::*#import"))) (name "*") (declared-name "*"))
                    (element (kind "derivation connection") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::_derivationConnection"))) (name "_derivationConnection")
                      (contains
                        (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::_derivationConnection::#derive"))) (name "#derive") (declared-name "#derive") (declared (properties (end true))))
                        (element (kind "interface end") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::_derivationConnection::#original"))) (name "#original") (declared-name "#original") (declared (properties (end true))))
                      )
                    )
                    (element (kind "dependency") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::dependency"))) (name "dependency") (declared-name "dependency"))
                    (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification"))) (name "engineSpecification") (declared-name "engineSpecification")
                      (contains
                        (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (name "drivePowerOutputRequirement") (declared-name "drivePowerOutputRequirement"))
                        (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engine1"))) (name "engine1") (declared-name "engine1"))
                        (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (name "engineMassRequirement") (declared-name "engineMassRequirement")
                          (contains
                            (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement::massRequired"))) (name "massRequired") (declared-name "massRequired") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                          )
                        )
                        (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (name "torqueGenerationRequirement") (declared-name "torqueGenerationRequirement"))
                      )
                    )
                    (element (kind "item def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::marketSurvey"))) (name "marketSurvey") (declared-name "marketSurvey"))
                    (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification"))) (name "vehicleSpecification") (declared-name "vehicleSpecification")
                      (contains
                        (element (kind "subject") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))) (name "vehicle") (declared-name "vehicle"))
                        (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements"))) (name "vehicleFuelEconomyRequirements") (declared-name "vehicleFuelEconomyRequirements")
                          (contains
                            (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::_documentation"))) (name ""))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::assumedCargoMass"))) (name "assumedCargoMass") (declared-name "assumedCargoMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                            (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement")
                              (contains
                                (element (kind "require constraint") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                              )
                            )
                            (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement")
                              (contains
                                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement::StatusInfo"))) (name "StatusInfo") (declared-name "StatusInfo") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))))
                                  (contains
                                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement::StatusInfo::originator"))) (name "originator") (declared-name "originator") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement::StatusInfo::owner"))) (name "owner") (declared-name "owner") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement::StatusInfo::status"))) (name "status") (declared-name "status") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                                  )
                                )
                                (element (kind "require constraint") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
                              )
                            )
                          )
                        )
                        (element (kind "requirement") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement")
                          (contains
                            (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "require constraint") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassActual"))) (name "fuelMassActual") (declared-name "fuelMassActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassMax"))) (name "fuelMassMax") (declared-name "fuelMassMax") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::massRequired"))) (name "massRequired") (declared-name "massRequired") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::mop"))) (name "mop") (declared-name "mop"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies"))) (name "WheelHubAssemblies") (declared-name "WheelHubAssemblies")
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1"))) (name "wheelHubAssy1") (declared-name "wheelHubAssy1") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (name "hub1") (declared-name "hub1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (name "wheel1") (declared-name "wheel1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2"))) (name "wheelHubAssy2") (declared-name "wheelHubAssy2") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (name "hub1") (declared-name "hub1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (name "wheel1") (declared-name "wheel1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3"))) (name "wheelHubAssy3") (declared-name "wheelHubAssy3") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (name "hub1") (declared-name "hub1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort"))) (name "shankCompositePort") (declared-name "shankCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))))
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))) (name "shankPort") (declared-name "shankPort") (declared (properties (composite true) (reference false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))))
                              (contains
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::shaftLength"))) (name "shaftLength") (declared-name "shaftLength") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 70)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::shaftLength"))) (role feature-value))))
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::threadDia"))) (name "threadDia") (declared-name "threadDia") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 14)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::threadDia"))) (role feature-value))))
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::threadPitch"))) (name "threadPitch") (declared-name "threadPitch") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "1.5")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::threadPitch"))) (role feature-value))))
                              )
                            )
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort1"))) (name "shankPort1") (declared-name "shankPort1") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort2"))) (name "shankPort2") (declared-name "shankPort2") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort3"))) (name "shankPort3") (declared-name "shankPort3") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (name "wheel1") (declared-name "wheel1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort"))) (name "lugNutCompositePort") (declared-name "lugNutCompositePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))) (name "lugNutPort") (declared-name "lugNutPort") (declared (properties (composite true) (reference false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
                              (contains
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort::threadDia"))) (name "threadDia") (declared-name "threadDia") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 14)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort::threadDia"))) (role feature-value))))
                                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort::threadPitch"))) (name "threadPitch") (declared-name "threadPitch") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "1.5")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort::threadPitch"))) (role feature-value))))
                              )
                            )
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort1"))) (name "lugNutPort1") (declared-name "lugNutPort1") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort2"))) (name "lugNutPort2") (declared-name "lugNutPort2") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                            (element (kind "port") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort3"))) (name "lugNutPort3") (declared-name "lugNutPort3") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
                          )
                        )
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals"))) (name "VehicleIndividuals") (declared-name "VehicleIndividuals")
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a"))) (name "a") (declared-name "a") (declared (properties (individual true) (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a"))) (name "t0_t2_a") (declared-name "t0_t2_a") (declared (properties (portion true) (composite true) (reference false) (portion-kind "timeslice")))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a"))) (name "t0_a") (declared-name "t0_a") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a::t0"))) (name "t0") (declared-name "t0") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a::t0_r"))) (name "t0_r") (declared-name "t0_r") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a::t0_v"))) (name "t0_v") (declared-name "t0_v") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                          (contains
                            (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a::t0_v::t0_fa"))) (name "t0_fa") (declared-name "t0_fa") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                              (contains
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a::t0_v::t0_fa::t0_leftFront"))) (name "t0_leftFront") (declared-name "t0_leftFront") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t0_a::t0_v::t0_fa::t0_rightFront"))) (name "t0_rightFront") (declared-name "t0_rightFront") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                              )
                            )
                          )
                        )
                      )
                    )
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a"))) (name "t1_a") (declared-name "t1_a") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a::t1"))) (name "t1") (declared-name "t1") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a::t1_r"))) (name "t1_r") (declared-name "t1_r") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a::t1_v"))) (name "t1_v") (declared-name "t1_v") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                          (contains
                            (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a::t1_v::t1_fa"))) (name "t1_fa") (declared-name "t1_fa") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                              (contains
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a::t1_v::t1_fa::t1_leftFront"))) (name "t1_leftFront") (declared-name "t1_leftFront") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t1_a::t1_v::t1_fa::t1_rightFront"))) (name "t1_rightFront") (declared-name "t1_rightFront") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                              )
                            )
                          )
                        )
                      )
                    )
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a"))) (name "t2_a") (declared-name "t2_a") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a::t2"))) (name "t2") (declared-name "t2") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a::t2_r"))) (name "t2_r") (declared-name "t2_r") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a::t2_v"))) (name "t2_v") (declared-name "t2_v") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                          (contains
                            (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a::t2_v::t2_fa"))) (name "t2_fa") (declared-name "t2_fa") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                              (contains
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a::t2_v::t2_fa::t2_leftFront"))) (name "t2_leftFront") (declared-name "t2_leftFront") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                                (element (kind "occurrence") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleIndividuals::a::t0_t2_a::t2_a::t2_v::t2_fa::t2_rightFront"))) (name "t2_rightFront") (declared-name "t2_rightFront") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))))
                              )
                            )
                          )
                        )
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration"))) (name "VehicleLogicalConfiguration") (declared-name "VehicleLogicalConfiguration")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
              (contains
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::_logical"))) (name "logical") (declared-name "logical"))
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (name "vehicleLogical") (declared-name "vehicleLogical") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (name "brakingSubsystem") (declared-name "brakingSubsystem") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (name "electricalGenerator") (declared-name "electricalGenerator") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                      (contains
                        (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator::generateElectricity"))) (name "generateElectricity") (declared-name "generateElectricity") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (name "steeringSystem") (declared-name "steeringSystem") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (name "torqueGenerator") (declared-name "torqueGenerator") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
                      (contains
                        (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator")))))
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation"))) (name "VehicleLogicalToPhysicalAllocation") (declared-name "VehicleLogicalToPhysicalAllocation")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation::PartsTree"))) (name "PartsTree") (declared-name "PartsTree"))
            (element (kind "allocation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation::vehicleLogicalToPhysicalAllocation"))) (name "vehicleLogicalToPhysicalAllocation") (declared-name "vehicleLogicalToPhysicalAllocation"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel"))) (name "VehicleSuperSetModel") (declared-name "VehicleSuperSetModel")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions"))) (name "VariationPointDefinitions") (declared-name "VariationPointDefinitions")
              (contains
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (name "TransmissionChoices") (declared-name "TransmissionChoices") (declared (properties (variation true)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices::transmissionAutomatic"))) (name "transmissionAutomatic") (declared-name "transmissionAutomatic") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices::transmissionManual"))) (name "transmissionManual") (declared-name "transmissionManual") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree"))) (name "VehiclePartsTree") (declared-name "VehiclePartsTree")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::*"))) (name "*") (declared-name "*"))
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily"))) (name "vehicleFamily") (declared-name "vehicleFamily") (declared (properties (abstract true) (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::driveshaft"))) (name "driveshaft") (declared-name "driveshaft") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (name "engine") (declared-name "engine") (declared (properties (variation true) (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine::engine4Cyl"))) (name "engine4Cyl") (declared-name "engine4Cyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine::engine6Cyl"))) (name "engine6Cyl") (declared-name "engine6Cyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
                          (contains
                            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine::engine6Cyl::cylinder"))) (name "cylinder") (declared-name "cylinder") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine6Cyl")))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (name "sunroof") (declared-name "sunroof") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (name "transmissionChoices") (declared-name "transmissionChoices") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification"))) (name "VehicleVerification") (declared-name "VehicleVerification")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::*#import"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::*#import2"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::*#import3"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VehicleConfiguration_b"))) (name "VehicleConfiguration_b") (declared-name "VehicleConfiguration_b"))
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions"))) (name "VerificationCaseDefinitions") (declared-name "VerificationCaseDefinitions")
              (contains
                (element (kind "verification def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::AccelerationTest"))) (name "AccelerationTest") (declared-name "AccelerationTest"))
                (element (kind "verification def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest"))) (name "MassTest") (declared-name "MassTest"))
                (element (kind "verification def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::ReliabilityTest"))) (name "ReliabilityTest") (declared-name "ReliabilityTest"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1"))) (name "VerificationCases1") (declared-name "VerificationCases1")
              (contains
                (element (kind "verification") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests"))) (name "massTests") (declared-name "massTests")
                  (contains
                    (element (kind "metadata usage") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::VerificationMethod"))) (name "VerificationMethod") (declared-name "VerificationMethod") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest"))))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::VerificationMethod::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest")))))
                      )
                    )
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::evaluatePassFail"))) (name "evaluatePassFail") (declared-name "evaluatePassFail") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest")))))
                    (element (kind "flow") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest")))))
                    (element (kind "objective") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::weighVehicle"))) (name "weighVehicle") (declared-name "weighVehicle") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest"))))
                      (contains
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::weighVehicle::massMeasured"))) (name "massMeasured") (declared-name "massMeasured") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest")))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem"))) (name "VerificationSystem") (declared-name "VerificationSystem")
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext"))) (name "verificationContext") (declared-name "verificationContext") (declared (properties (composite true) (reference false) (ordered false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massTests"))) (name "massTests") (declared-name "massTests"))
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem"))) (name "massVerificationSystem") (declared-name "massVerificationSystem") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::operator"))) (name "operator") (declared-name "operator") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                          (contains
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::operator::massTests.evaluatePassFail"))) (name "massTests.evaluatePassFail") (declared-name "massTests.evaluatePassFail"))
                          )
                        )
                        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::scale"))) (name "scale") (declared-name "scale") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                          (contains
                            (element (kind "action") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::scale::massTests.weighVehicle"))) (name "massTests.weighVehicle") (declared-name "massTests.weighVehicle"))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::vehicle_UnitUnderTest"))) (name "vehicle_UnitUnderTest") (declared-name "vehicle_UnitUnderTest") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints"))) (name "Views_Viewpoints") (declared-name "Views_Viewpoints")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews"))) (name "VehicleViews") (declared-name "VehicleViews")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::*"))) (name "*") (declared-name "*"))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::*#import"))) (name "*") (declared-name "*"))
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::*#import2"))) (name "*") (declared-name "*"))
                (element (kind "view") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::vehiclePartsTree_Safety"))) (name "vehiclePartsTree_Safety") (declared-name "vehiclePartsTree_Safety")
                  (contains
                    (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::vehiclePartsTree_Safety::**"))) (name "**") (declared-name "**") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView")))))
                    (element (kind "filter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::vehiclePartsTree_Safety::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "Safety")))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions"))) (name "ViewDefinitions") (declared-name "ViewDefinitions")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::*"))) (name "*") (declared-name "*"))
                (element (kind "view def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::NestedView"))) (name "NestedView") (declared-name "NestedView"))
                (element (kind "view def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsInterconnection"))) (name "PartsInterconnection") (declared-name "PartsInterconnection"))
                (element (kind "view def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView"))) (name "PartsTreeView") (declared-name "PartsTreeView")
                  (contains
                    (element (kind "filter") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "SysML::PartUsage")))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView")))))
                  )
                )
                (element (kind "view def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::RelationshipView"))) (name "RelationshipView") (declared-name "RelationshipView"))
                (element (kind "view def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::TableView"))) (name "TableView") (declared-name "TableView"))
                (element (kind "view def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::TreeView"))) (name "TreeView") (declared-name "TreeView")
                  (contains
                    (element (kind "view rendering") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::TreeView::asTreeDiagram"))) (name "asTreeDiagram") (declared-name "asTreeDiagram") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::TreeView")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions"))) (name "ViewpointDefinitions") (declared-name "ViewpointDefinitions")
              (contains
                (element (kind "viewpoint def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::BehaviorViewpoint"))) (name "BehaviorViewpoint") (declared-name "BehaviorViewpoint"))
                (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::SafetyEngineer"))) (name "SafetyEngineer") (declared-name "SafetyEngineer") (declared))
                (element (kind "viewpoint def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::SafetyViewpoint"))) (name "SafetyViewpoint") (declared-name "SafetyViewpoint"))
                (element (kind "concern def") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety"))) (name "VehicleSafety") (declared-name "VehicleSafety")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety")))))
                    (element (kind "stakeholder") (id (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety::se"))) (name "se") (declared-name "se") (effective (featuring-type (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety")))))
                  )
                )
              )
            )
          )
        )
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::unresolved_allocate_source"))) (name "unresolved_allocate_source") (declared-name "unresolved_allocate_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation::unresolved_allocate_target"))) (name "unresolved_allocate_target") (declared-name "unresolved_allocate_target"))
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::_refinement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::bumper::Safety"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::bumper"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::keylessEntry::Security"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::keylessEntry"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::alarm::Security"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::alarm"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::driverAirBag::Safety"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::driverAirBag"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::seatBelt::Safety"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::seatBelt"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement::StatusInfo"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::_logical"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::VerificationMethod"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety::_documentation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety"))))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (connect (source-expression "vehicleSoftware::vehicleController::cruiseController::cruiseControlPort") (target-expression "vehicleSoftware::vehicleController::controlPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (connect (source-expression "engine::fuelCmdPort") (target-expression "fuelCmdPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (connect (source-expression "engine::ignitionCmdPort") (target-expression "ignitionCmdPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (connect (source-expression "rearAxleAssembly::rearWheel1::wheelToRoadPort") (target-expression "vehicleToRoadPort::wheelToRoadPort1") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (connect (source-expression "rearAxleAssembly::rearWheel2::wheelToRoadPort") (target-expression "vehicleToRoadPort::wheelToRoadPort2") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (connect (source-expression "shaftPort_d") (target-expression "differential::shaftPort_d") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (connect (source-expression "vehicle_b::setSpeedPort") (target-expression "vehicleSoftware::vehicleController::cruiseController::setSpeedPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (connect (source-expression "engine::drivePwrPort") (target-expression "transmission::clutchPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b") (interface-usage true) (interface-type "EngineToTransmissionInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (connect (source-expression "fuelTank::fuelOutPort") (target-expression "engine::fuelInPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b") (interface-usage true) (interface-type "FuelInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (connect (source-expression "speedSensor::speedSensorPort") (target-expression "vehicleSoftware::vehicleController::cruiseController::speedSensorPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::flyWheelPort"))) (connect (source-expression "starterMotor::gearPort") (target-expression "engine::flyWheelPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (connect (source-expression "vehicleSoftware::vehicleController::controlPort") (target-expression "engine::engineControlPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort::~DrivePwrPort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::~FuelPort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (connect (source-expression "driveshaft::shaftPort_c") (target-expression "rearAxleAssembly::shaftPort_d") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (connect (source-expression "differential::leftDiffPort") (target-expression "rearAxle::leftHalfAxle::leftAxleToDiffPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (connect (source-expression "differential::rightDiffPort") (target-expression "rearAxle::rightHalfAxle::rightAxleToDiffPort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::shankCompositePort"))) (connect (source-expression "rearWheel1::lugNutCompositePort") (target-expression "rearAxle::leftHalfAxle::shankCompositePort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly") (interface-usage true) (interface-type "WheelHubInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::shankCompositePort"))) (connect (source-expression "rearWheel2::lugNutCompositePort") (target-expression "rearAxle::rightHalfAxle::shankCompositePort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly") (interface-usage true) (interface-type "WheelHubInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (connect (source-expression "transmission::shaftPort_a") (target-expression "driveshaft::shaftPort_b") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1::shankCompositePort"))) (connect (source-expression "wheel1::lugNutCompositePort") (target-expression "hub1::shankCompositePort") (container-prefix "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1") (interface-usage true) (interface-type "WheelHubInterface")))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::marketSurvey"))))
    (derivation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque::transmissionTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque::transmissionTorque"))) (flow (source-expression "amplifyTorque::transmissionTorque") (target-expression "transferTorque::transmissionTorque")))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::engineTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque::engineTorque"))) (flow (source-expression "generateTorque::engineTorque") (target-expression "amplifyTorque::engineTorque")))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque::driveshaftTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque::driveshaftTorque"))) (flow (source-expression "transferTorque::driveshaftTorque") (target-expression "distributeTorque::driveshaftTorque")))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::enterVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::closeDoor_in"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::openDoor_in"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::enterVehicle"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::closeDoor_out"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::lockDoor_out"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::exitVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::closeDoor_out"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::start"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::a"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::trigger"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::b"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::c"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::c"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::_verdict"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::start"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::a"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::trigger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::b"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::start"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driveVehicleToDestination"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::providePower"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driverGetInVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::passenger1GetInVehicle"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driverGetOutOfVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::passenger1GetOutOfVehicle"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::start"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driverGetInVehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::generateTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::applyParkingBrake"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::controlDirection"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::performSelfTest"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::provideBraking"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::providePower"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::senseTemperature"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::closeDoor_in"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::enterVehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle::openDoor_in"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::closeDoor_out"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::exitVehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle::lockDoor_out"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::a"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::b"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::c"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger::trigger"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::transportPassenger"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::evaluatePassFail"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massTests"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort::~AxlePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxleToWheelPort::~AxleToWheelPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxleToWheelPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort::~ControlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort::~CruiseControlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort::~DiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort::~DrivePwrPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort::~DriverCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort::~FuelCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::~FuelPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort::~GearPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort::~HandPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort::~IgnitionCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::~LugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::~LugNutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort::~PwrCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort::~SetSpeedPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a::~ShaftPort_a"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b::~ShaftPort_b"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c::~ShaftPort_c"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d::~ShaftPort_d"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::~ShankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::~ShankPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort::~SpeedSensorPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort::~StatusPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort::~VehicleToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToAxlePort::~WheelToAxlePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToAxlePort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort::~WheelToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::cargoMass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::dryMass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank::mass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels::diameter"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body::color"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::cargoMass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::dryMass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank::fuelMassMax"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank::mass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::diameter"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::diameter"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::controllerStates"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleStates"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement::massActual"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement::massRequired"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::massActual"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::massRequired"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::shaftLength"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::shaftLength"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::threadDia"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadDia"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort::threadPitch"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadPitch"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort::threadDia"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadDia"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort::threadPitch"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadPitch"))))
    (satisfy (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))))
    (satisfy (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxleAssembly_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxle_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::RearAxleAssembly_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Road_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::VehicleRoadContext_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Vehicle_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsInterconnection"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::NestedView"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::TreeView"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement::generateTorque"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engine1"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engine1"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engine1"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engine1"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinders"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinders"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder3"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinders"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder4"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinders"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::partMasses"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort3"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort3"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::degraded"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::maintenance"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::degraded"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::normal"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::healthStates::maintenance"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::starting"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::starting"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::operatingStates::on"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::off"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::on"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::on"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates::operatingStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::driverStates::wait"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface::p1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface::p2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort::~DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface::fuelInPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::~FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface::fuelOutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::lugNutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::maxTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::shankPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort::~SetSpeedPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort::~SpeedSensorPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort::~ControlPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::~FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::~FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort::~DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates::controller"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort::driverCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort::fuelCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort::fuel"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort::ignitionCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort::pwrCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort::ignitionCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort::pwrCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle::vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle::vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::AverageTravelTimePerDistance::scenario"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::ComputeBSFC::engine"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::IdlingFuelConsumptionPerTime::engine"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::TraveledDistance::scenario"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::fuelEconomyAnalysis::scenario"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::sendFuelCmd::_payload"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl1::vehicle_b::sendSensedSpeed::_payload"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::sendFuelCmd::_payload"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::CruiseControl2::vehicle_b::sendSensedSpeed::_payload"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort::~SetSpeedPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engine1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicle"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1::shankCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1::lugNutCompositePort"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation::vehicleLogicalToPhysicalAllocation"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::AllocationDefinitions::LogicalToPhysical"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices::transmissionAutomatic"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionAutomatic"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices::transmissionManual"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionManual"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine::engine4Cyl"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine4Cyl"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine::engine6Cyl"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine6Cyl"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine::engine6Cyl::cylinder"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Sunroof"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionChoices"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews::vehiclePartsTree_Safety"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety::se"))) (to (node (document "d0") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::SafetyEngineer"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::fork2") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driveVehicleToDestination"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::fork3") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::driverGetOutOfVehicle"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join1") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join1"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join1") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::trigger"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join2") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::fork3"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join2") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join2"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join3") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::_verdict"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join3") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join3"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::passenger1GetInVehicle") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join1"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::passenger1GetOutOfVehicle") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join3"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::providePower") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::join2"))
    (flow (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::trigger") (target-qualified "SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger_1::fork2"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::a::driverGetInVehicle::closeDoor_in"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::a::driverGetInVehicle::enterVehicle"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::a::driverGetInVehicle::openDoor_in"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::a::driverGetInVehicle::unlockDoor_in"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::b::driveVehicleToDestination"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::c::driverGetOutOfVehicle::closeDoor_out"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::c::driverGetOutOfVehicle::exitVehicle"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::c::driverGetOutOfVehicle::lockDoor_out"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::driver") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::driver::transportPassenger::c::driverGetOutOfVehicle::openDoor_out"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::a::passenger1GetInVehicle::closeDoor_in"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::a::passenger1GetInVehicle::enterVehicle"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::a::passenger1GetInVehicle::openDoor_in"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::a::passenger1GetInVehicle::unlockDoor_in"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::c::passenger1GetOutOfVehicle::closeDoor_out"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::c::passenger1GetOutOfVehicle::exitVehicle"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::c::passenger1GetOutOfVehicle::lockDoor_out"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::passenger1::transportPassenger::c::passenger1GetOutOfVehicle::openDoor_out"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1") (target-qualified "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1::transportPassenger::trigger"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver::startVehicle::trigger2"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver::startVehicle::turnVehicleOn"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle::startVehicle::sendStatus"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle::startVehicle::trigger1"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::providePower::transferTorque"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::providePower::distributeTorque"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission") (target-qualified "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::providePower::amplifyTorque"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::operator") (target-qualified "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::operator::massTests::evaluatePassFail"))
    (perform (status pending) (document "d0") (source-qualified "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::scale") (target-qualified "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::scale::massTests::weighVehicle"))
  )
  (pending-expression-relationships
    (allocate (status pending-expression) (document "d0") (source-expression "ActionTree::providePower::generateToAmplify") (target-expression "engineToTransmissionInterface") (container-prefix "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))
    (allocate (status pending-expression) (document "d0") (source-expression "vehicleLogical") (target-expression "vehicle_b") (container-prefix "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation"))
    (connection (status pending-expression) (document "d0") (source-expression "driver::handPort") (target-expression "vehicle_b_1::ignitionCmdPort") (container-prefix "SimpleVehicleModel::MissionContext::missionContext"))
    (connection (status pending-expression) (document "d0") (source-expression "road") (target-expression "vehicle_b_1::vehicleToRoadPort") (container-prefix "SimpleVehicleModel::MissionContext::missionContext"))
  )
)
~~~
