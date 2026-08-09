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
package SimpleVehicleModel {
    // 2023-02 release
    public import Definitions::*;
    public import ISQ::*;
    package Definitions {
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
        package PartDefinitions {
            part def Vehicle {
                attribute mass :> ISQ::mass;
                attribute dryMass :> ISQ::mass;
                attribute cargoMass :> ISQ::mass;
                attribute position :> ISQ::length;
                attribute velocity :> ISQ::speed;
                attribute acceleration :> ISQ::acceleration;
                attribute electricalPower :> ISQ::power;
                attribute Tmax :> ISQ::temperature;
                attribute maintenanceTime : Time::DateTime;
                attribute brakePedalDepressed : Boolean;
                port ignitionCmdPort : IgnitionCmdPort;
                port pwrCmdPort : PwrCmdPort;
                port vehicleToRoadPort : VehicleToRoadPort;
                port statusPort : StatusPort;
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
            part def Engine {
                attribute mass :> ISQ::mass;
                attribute peakHorsePower :> ISQ::power;
                attribute fuelEfficiency : Real;
                attribute cost : Real;
                attribute displacement :> ISQ::volume;
                port engineControlPort : ~ControlPort;
                port fuelInPort : ~FuelPort;
                port fuelCmdPort : FuelCmdPort;
                port drivePwrPort : DrivePwrPort;
                port ignitionCmdPort : IgnitionCmdPort;
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
            part def StarterMotor {
                port gearPort : GearPort;
            }
            part def Cylinder;
            part def Transmission {
                attribute gearRatio : Real;
                port clutchPort : ~DrivePwrPort;
                exhibit state transmissionStates;
            }
            part def Driveshaft;
            part def AxleAssembly;
            part def Axle {
                attribute mass :> ISQ::mass;
            }
            part def FrontAxle :> Axle {
                attribute steeringAngle :> ISQ::angularMeasure;
            }
            part def HalfAxle {
                port shankCompositePort : ShankCompositePort { }
            }
            part def Differential;
            part def Wheel {
                attribute diameter : LengthValue;
                port lugNutCompositePort : LugNutCompositePort;
            }
            part def Hub {
                port shankCompositePort : ShankCompositePort;
            }
            abstract part def Software;
            part def VehicleSoftware :> Software;
            part def VehicleController :> Software {
                port controlPort : ControlPort;
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
            part def CruiseController :> Software {
                port setSpeedPort : ~SetSpeedPort;
                port speedSensorPort : ~SpeedSensorPort;
                port cruiseControlPort : CruiseControlPort;
                exhibit state cruiseControllerStates;
            }
            part def SpeedSensor {
                port speedSensorPort : SpeedSensorPort;
            }
            part def FuelTank {
                attribute mass :> ISQ::mass;
                ref item fuel : Fuel {
                    attribute :>> fuelMass;
                }
                attribute fuelKind : FuelKind;
                attribute fuelMassMax :> ISQ::mass;
                assert constraint fuelConstraint {
                    = fuel.fuelMass <= fuelMassMax;
                }
                port fuelOutPort : FuelPort;
                port fuelInPort : ~FuelPort;
            }
            part def BodyAssy;
            part def Body {
                attribute color : Colors;
            }
            part def Thermostat;
            part def WaterHose;
            part def Road {
                attribute incline : Real;
                attribute friction : Real;
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
        package PortDefinitions {
            port def IgnitionCmdPort {
                in item ignitionCmd : IgnitionCmd;
            }
            port def StatusPort;
            port def GearPort;
            port def PwrCmdPort {
                in item pwrCmd : PwrCmd;
            }
            port def FuelCmdPort :> PwrCmdPort {
                in item fuelCmd : FuelCmd redefines pwrCmd;
            }
            port def FuelPort {
                out item fuel : Fuel;
            }
            port def DrivePwrPort {
                out torque : Torque;
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

            port def LugNutCompositePort {
                port lugNutPort : LugNutPort [*];
            }
            port def ShankCompositePort {
                port shankPort : ShankPort [*];
            }
            port def LugNutPort {
                attribute threadDia;
                attribute threadPitch;
            }
            port def ShankPort {
                attribute threadDia;
                attribute threadPitch;
                attribute shaftLength;
            }

            port def VehicleToRoadPort;
            port def ControlPort;
            port def CruiseControlPort :> ControlPort;
            port def SpeedSensorPort;
            port def SetSpeedPort;

            port def DriverCmdPort {
                out item driverCmd : DriverCmd [*];
            }
            port def HandPort :> DriverCmdPort {
                out item ignitionCmd : IgnitionCmd subsets driverCmd;
                out item pwrCmd : PwrCmd subsets driverCmd;
            }
        }
        package ItemDefinitions {
            item def PwrCmd {
                attribute throttleLevel : Real;
            }
            item def FuelCmd :> PwrCmd;
            item def Fuel {
                attribute fuelMass :> ISQ::mass;
            }
            item def SensedSpeed {
                attribute speed :> ISQ::speed;
            }
        }
        package SignalDefinitions {
            item def Cmd { }
            item def DriverCmd;
            item def IgnitionCmd :> DriverCmd {
                attribute ignitionOnOff : IgnitionOnOff;
            }
            item def EngineStatus;

            attribute def VehicleStartSignal;
            attribute def VehicleOnSignal;
            attribute def VehicleOffSignal;
            attribute def StartSignal;
            attribute def OffSignal;
            attribute def OverTemp;
            attribute def ReturnToNormal;
            attribute def SetSpeed :> Real;
        }
        package InterfaceDefinitions {
            interface def EngineToTransmissionInterface {
                end p1 : DrivePwrPort;
                end p2 : DrivePwrPort;
                flow p1;
            }
            interface def FuelInterface {
                end fuelOutPort : FuelPort;
                end fuelInPort : FuelPort;
                flow of;
            }

            interface def WheelFastenerInterface {
                end lugNutPort : LugNutPort;
                end shankPort : ShankPort;
                attribute maxTorque : Torque;
                constraint {
                    = lugNutPort.threadDia == shankPort.threadDia;
                }
            }
            interface def WheelHubInterface {
                end lugNutCompositePort : LugNutCompositePort;
                end shankCompositePort : ShankCompositePort;
                interface wheelFastenerInterface : WheelFastenerInterface [5] connect lugNutCompositePort.lugNutPort to shankCompositePort.shankPort;
            }
        }
        package AllocationDefinitions {
            allocation def LogicalToPhysical {
                end #logical logicalEnd;
                end #physical physicalEnd;
            }
        }
        package ActionDefinitions {
            action def ProvidePower {
                in item pwrCmd : PwrCmd;
                out wheelToRoadTorque : Torque [2];
            }
            action def GenerateTorque {
                in item fuelCmd : FuelCmd;
                out engineTorque : Torque;
            }
            action def AmplifyTorque {
                in engineTorque : Torque;
                out transmissionTorque : Torque;
            }
            action def TransferTorque {
                in transmissionTorque : Torque;
                out driveshaftTorque : Torque;
            }
            action def DistributeTorque {
                in driveshaftTorque : Torque;
                out wheelToRoadTorque : Torque [2];
            }
            action def PerformSelfTest;
            action def ApplyParkingBrake;
            action def SenseTemperature {
                out temp : ISQ::TemperatureValue;
            }
        }
        package StateDefinitions {
            state def VehicleStates;
            state def ControllerStates;
            state def CruiseControllerStates;
        }
        package RequirementDefinitions {
            requirement def MassRequirement {
                doc /*The actual mass shall be less than the required mass*/
                attribute massRequired :> ISQ::mass;
                attribute massActual :> ISQ::mass;
                require constraint {
                    = massActual <= massRequired;
                }
            }
            requirement def ReliabilityRequirement {
                doc /*The actual reliability shall be greater than the required reliability*/
                attribute reliabilityRequired : Real;
                attribute reliabilityActual : Real;
                require constraint {
                    = reliabilityActual >= reliabilityRequired;
                }
            }
            requirement def TorqueGenerationRequirement {
                doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
                subject generateTorque : ActionDefinitions::GenerateTorque;
            }
            requirement def DrivePowerOutputRequirement {
                doc /* The engine shall provide a connection point to transfer torque to the transmission.*/
            }
            requirement def FuelEconomyRequirement {
                doc /* The vehicle shall maintain an average fuel economomy of at least x miles per gallon for the nominal 
                driving scenario */
                attribute actualFuelEconomy :> distancePerVolume;
                attribute requiredFuelEconomy :> distancePerVolume;
                require constraint {
                    = actualFuelEconomy >= requiredFuelEconomy;
                }
            }
        }
        package AttributeDefinitions {
            public import ScalarValues::*;
            public import Quantities::*;
            public import MeasurementReferences::DerivedUnit;
            public import SIPrefixes::kilo;
            // Numerical Functions provides basic operators such as Sum expression
            public import NumericalFunctions::*;
            public import SI::*;
            public import USCustomaryUnits::*;
            alias Torque for ISQ::TorqueValue;

            enum def Colors {
                enum black;
                enum grey;
                enum red;
            }
            enum def DiameterChoices :> ISQ::LengthValue {
                enum = 60 [mm];
                enum = 80 [mm];
                enum = 100 [mm];
            }
            attribute cylinderDiameter : DiameterChoices = 80 [mm];
            enum def IgnitionOnOff {
                enum on;
                enum off;
            }
            enum def FuelKind {
                enum gas;
                enum diesel;
            }

            distancePerVolume:> scalarQuantities = distance / volume;
            timePerDistance:> scalarQuantities = time / distance;
            volumePerDistance:> scalarQuantities = volume / distance;
            volumePerTime:> scalarQuantities = volume / time;

            // kpl is approx .425 * mpg
            kpl: DerivedUnit = km / L;
            rpm: DerivedUnit = 1 / SI::min;
            kW: DerivedUnit = kilo * W;
        }
        package IndividualDefinitions {
            individual def VehicleRoadContext_1 :> GenericContext::Context;
            individual def Vehicle_1 :> Vehicle;
            individual def FrontAxleAssembly_1 :> AxleAssembly;
            individual def FrontAxle_1 :> FrontAxle;
            individual def Wheel_1 :> Wheel;
            individual def Wheel_2 :> Wheel;
            individual def RearAxleAssembly_1 :> AxleAssembly;
            individual def Road_1 :> Road;
        }
        package MetadataDefinitions {
            public import AnalysisTooling::*;
            metadata def Safety {
                attribute isMandatory : Boolean;
            }
            metadata def Security;
        }
        package KeyWord_MetadataDefinitions {
            public import Metaobjects::SemanticMetadata;

            // the following is used to define the key word failureMode
            state failureModes [*] nonunique;

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
                attribute time : TimeValue;
                attribute spatialCF : CartesianSpatial3dCoordinateFrame [1] {
                    :>> mRefs = (m, m, m);
                }
                attribute velocityCF : CartesianVelocity3dCoordinateFrame [1] = spatialCF/s;
                attribute accelarationCF : CartesianAcceleration3dCoordinateFrame [1] = velocityCF/s;
            }
        }
    }

    package VehicleLogicalConfiguration {
        package PartsTree {
            #logical part vehicleLogical : Vehicle {
                part torqueGenerator : TorqueGenerator {
                    action generateTorque;
                }
                part electricalGenerator : ElectricalGenerator {
                    action generateElectricity;
                }
                part steeringSystem : SteeringSubsystem;
                part brakingSubsystem : BrakingSubsystem;
            }
        }
    }
    package VehicleLogicalToPhysicalAllocation {
        public import VehicleConfigurations::VehicleConfiguration_b::PartsTree::**;
        public import VehicleLogicalConfiguration::PartsTree::*;

        allocation vehicleLogicalToPhysicalAllocation : LogicalToPhysical allocate vehicleLogical to vehicle_b {
            allocate vehicleLogical.torqueGenerator to vehicle_b.engine {
                allocate vehicleLogical.torqueGenerator.generateTorque to vehicle_b.engine.generateTorque;
            }
            allocate vehicleLogical.electricalGenerator to vehicle_b.engine {
                allocate vehicleLogical.electricalGenerator.generateElectricity to vehicle_b.engine.alternator.generateElectricity;
            }
        }
    }
    package VehicleConfigurations {
        package VehicleConfiguration_a {
            package PartsTree {
                part vehicle_a : Vehicle {
                    attribute mass redefines Vehicle::mass = dryMass+cargoMass+fuelTank.fuel.fuelMass;
                    attribute dryMass redefines Vehicle::dryMass = sum(partMasses);
                    attribute redefines Vehicle::cargoMass = 0 [kg];
                    attribute partMasses :> ISQ::mass [*] nonunique;
                    part fuelTank : FuelTank {
                        attribute redefines mass = 75[kg];
                        ref item redefines fuel {
                            attribute redefines fuelMass = 50[kg];
                        }
                    }
                    part frontAxleAssembly : AxleAssembly {
                        attribute mass :> ISQ::mass = 800[kg];
                        part frontAxle : Axle;
                        part frontWheels : Wheel [2];
                    }
                    part rearAxleAssembly : AxleAssembly {
                        attribute mass :> ISQ::mass = 875[kg];
                        attribute driveTrainEfficiency : Real = 0.6;
                        part rearAxle : Axle;
                        part rearWheels : Wheel [2] {
                            attribute redefines diameter;
                        }
                    }
                }
            }
            package ActionTree { }
            package Requirements { }
        }
        package VehicleConfiguration_b {
            //Shapes library for simple geometry
            public import ShapeItems::Box;
            public import ParametersOfInterestMetadata::mop;
            public import ModelingMetadata::*;
            // incudes status info

            package PartsTree {
                part vehicle_b : Vehicle {
                    #mop attribute mass redefines mass = dryMass+cargoMass+fuelTank.fuel.fuelMass;
                    attribute dryMass redefines dryMass = sum(partMasses);
                    attribute redefines cargoMass default = 0 [kg];
                    attribute partMasses = (fuelTank.mass,frontAxleAssembly.mass,rearAxleAssembly.mass,engine.mass,transmission.mass,driveshaft.mass);
                    attribute avgFuelEconomy :> distancePerVolume;
                    port fuelCmdPort : FuelCmdPort redefines pwrCmdPort {
                        in item fuelCmd redefines pwrCmd;
                    }
                    port setSpeedPort : ~SetSpeedPort;
                    port vehicleToRoadPort redefines vehicleToRoadPort {
                        port wheelToRoadPort1 : WheelToRoadPort;
                        port wheelToRoadPort2 : WheelToRoadPort;
                    }
                    perform :>> ActionTree::providePower;
                     redefines providePower;
                    perform :>> ActionTree::performSelfTest;
                     redefines performSelfTest;
                    perform :>> ActionTree::applyParkingBrake;
                     redefines applyParkingBrake;
                    perform :>> ActionTree::senseTemperature;
                     redefines senseTemperature;
                    exhibit state vehicleStates redefines vehicleStates;

                    // Example vehicle with simple enveloping shape that is a solid 
                    item :> envelopingShapes : Box [1] {
                        length1 :>> length = 4800 [mm];
                        width1 :>> width = 1840 [mm];
                        height1 :>> height = 1350 [mm];
                    }

                    part fuelTank : FuelTank {
                        attribute redefines mass = 75[kg];
                        ref item redefines fuel {
                            attribute redefines fuelMass = 60[kg];
                        }
                        attribute redefines fuelMassMax = 60 [kg];
                    }
                    part frontAxleAssembly : AxleAssembly {
                        attribute mass :> ISQ::mass = 800[kg];
                        port shaftPort_d : ShaftPort_d;
                        part frontAxle : FrontAxle;
                        part frontWheels : Wheel [2];
                    }

                    part rearAxleAssembly : AxleAssembly {
                        attribute mass :> ISQ::mass = 875[kg];
                        attribute driveTrainEfficiency : Real = 0.6;
                        port shaftPort_d : ShaftPort_d;
                        perform :>> providePower.distributeTorque;
                        part rearWheel1 : Wheel {
                            attribute redefines diameter;
                            port wheelToRoadPort : WheelToRoadPort;
                            port lugNutCompositePort :>> lugNutCompositePort {
                                port lugNutPort :>> lugNutPort [5];
                            }
                        }
                        part rearWheel2 : Wheel {
                            attribute redefines diameter;
                            port wheelToRoadPort : WheelToRoadPort;
                            port lugNutCompositePort :>> lugNutCompositePort {
                                port lugNutPort :>> lugNutPort [5];
                            }
                        }
                        part differential : Differential {
                            port shaftPort_d : ShaftPort_d;
                            port leftDiffPort : DiffPort;
                            port rightDiffPort : DiffPort;
                        }
                        part rearAxle {
                            part leftHalfAxle : HalfAxle {
                                port leftAxleToDiffPort : AxlePort;
                                port shankCompositePort :>> shankCompositePort {
                                    port shankPort :>> shankPort [5];
                                }
                            }
                            part rightHalfAxle : HalfAxle {
                                port rightAxleToDiffPort : AxlePort;
                                port shankCompositePort :>> shankCompositePort {
                                    port shankPort :>> shankPort [5];
                                }
                            }
                        }

                        bind shaftPort_d = differential.shaftPort_d;
                        connect differential.leftDiffPort to rearAxle.leftHalfAxle.leftAxleToDiffPort;
                        connect differential.rightDiffPort to rearAxle.rightHalfAxle.rightAxleToDiffPort;

                        interface wheelToleftHalAxleInterface : WheelHubInterface connect [1] rearWheel1.lugNutCompositePort to [1] rearAxle.leftHalfAxle.shankCompositePort;
                        interface wheelTorightHalAxleInterface : WheelHubInterface connect [1] rearWheel2.lugNutCompositePort to [1] rearAxle.rightHalfAxle.shankCompositePort;
                    }
                    part starterMotor : StarterMotor;
                    part engine : Engine {
                        perform :>> providePower.generateTorque;
                         redefines generateTorque;
                        part cylinders : Cylinder [4..6];
                        part alternator {
                            action generateElectricity;
                        }
                        ::engineSpecification by vehicle_b.engine{
                            requirement torqueGenerationRequirement :>> torqueGenerationRequirement{
                                subject generateTorque redefines generateTorque = vehicle_b.engine.generateTorque;
                            }
                            requirement drivePowerOuputRequirement :>> drivePowerOutputRequirement{
                                port torqueOutPort redefines torqueOutPort=vehicle_b.engine.drivePwrPort;
                            }
                        }
                    }
                    part transmission : Transmission {
                        attribute mass :> ISQ::mass = 100[kg];
                        port shaftPort_a : ShaftPort_a;
                        perform :>> providePower.amplifyTorque;
                    }
                    part driveshaft : Driveshaft {
                        attribute mass :> ISQ::mass = 100[kg];
                        port shaftPort_b : ShaftPort_b;
                        port shaftPort_c : ShaftPort_c;
                        perform :>> providePower.transferTorque;
                    }
                    part vehicleSoftware : VehicleSoftware {
                        part vehicleController : VehicleController {
                            exhibit state controllerStates redefines controllerStates;
                            part cruiseController : CruiseController;
                        }
                    }
                    part speedSensor : SpeedSensor;

                    // parts in bodyAssy and interioer are marked as safety or security features
                    part bodyAssy : BodyAssy {
                        part body : Body {
                            attribute :>> color = Colors::red;
                        }
                        part bumper {
                            @Safety {
                                isMandatory = true;
                            }
                        }
                        part keylessEntry {
                            @Security;
                        }
                    }
                    part interior {
                        part alarm {
                            @Security;
                        }
                        part seatBelt [2] {
                            @Safety {
                                isMandatory = true;
                            }
                        }
                        part frontSeat [2];
                        part driverAirBag {
                            @Safety {
                                isMandatory = false;
                            }
                        }
                    }

                    //connections
                    bind engine.fuelCmdPort = fuelCmdPort;

                    interface engineToTransmissionInterface : EngineToTransmissionInterface connect engine.drivePwrPort to transmission.clutchPort;

                    interface fuelInterface : FuelInterface connect fuelTank.fuelOutPort to engine.fuelInPort;

                    allocate ActionTree::providePower.generateToAmplify to engineToTransmissionInterface;

                    bind engine.ignitionCmdPort = ignitionCmdPort;
                    connect starterMotor.gearPort to engine.flyWheelPort;
                    connect vehicleSoftware.vehicleController.controlPort to engine.engineControlPort;
                    bind vehicle_b.setSpeedPort = vehicleSoftware.vehicleController.cruiseController.setSpeedPort;
                    connect speedSensor.speedSensorPort to vehicleSoftware.vehicleController.cruiseController.speedSensorPort;
                    bind vehicleSoftware.vehicleController.cruiseController.cruiseControlPort = vehicleSoftware.vehicleController.controlPort;
                    connect transmission.shaftPort_a to driveshaft.shaftPort_b;
                    connect driveshaft.shaftPort_c to rearAxleAssembly.shaftPort_d;
                    bind rearAxleAssembly.rearWheel1.wheelToRoadPort = vehicleToRoadPort.wheelToRoadPort1;
                    bind rearAxleAssembly.rearWheel2.wheelToRoadPort = vehicleToRoadPort.wheelToRoadPort2;

                    ::vehicleSpecification by vehicle_b{
                        requirement vehicleMassRequirement:>>vehicleMassRequirement{
                            attribute redefines massActual=vehicle_b.mass;
                            attribute redefines fuelMassActual = vehicle_b.fuelTank.fuel.fuelMass;
                        }
                    }
                }
            }
            package ActionTree {
                action providePower : ProvidePower {
                    in item fuelCmd : FuelCmd redefines pwrCmd;
                    out wheelToRoadTorque redefines wheelToRoadTorque [2] = distributeTorque.wheelToRoadTorque;
                    action generateTorque : GenerateTorque {
                        in
                        item = providePower.fuelCmd;
                    }
                    action amplifyTorque : AmplifyTorque;
                    action transferTorque : TransferTorque;
                    action distributeTorque : DistributeTorque;

                    //named flow
                    flow generateToAmplify from generateTorque.engineTorque to amplifyTorque.engineTorque;
                    //unnamed flows
                    flow amplifyTorque;
                    flow transferTorque;
                }
                action performSelfTest : PerformSelfTest;
                action applyParkingBrake : ApplyParkingBrake;
                action senseTemperature : SenseTemperature;
            }
            package DiscreteInteractions {
                package Sequence {
                    part def Driver {
                        port p1;
                        port p2;
                    }

                    part part0 {
                        perform action startVehicle {
                            action turnVehicleOn;
                            send ignitionCmd via driver.p1 {
                                in ignitionCmd : IgnitionCmd;
                            }
                            action trigger1;
                            accept ignitionCmd:IgnitionCmd via vehicle.ignitionCmdPort;
                            flow of;
                            action startEngine {
                                in item ignitionCmd : IgnitionCmd;
                                out item es : EngineStatus;
                            }
                            flow of;
                            action sendStatus;
                            send es via vehicle.statusPort {
                                in es : EngineStatus;
                            }
                            action trigger2;
                            accept es:EngineStatus via driver.p2;
                        }
                        part driver : Driver {
                            perform :>> startVehicle.turnVehicleOn;
                            perform :>> startVehicle.trigger2;
                            event occurrence driverReady;
                        }
                        part vehicle : Vehicle {
                            perform :>> startVehicle.trigger1;
                            perform :>> startVehicle.sendStatus;
                            event occurrence doorClosed;
                        }
                        first vehicle.doorClosed then driver.driverReady;
                        message of;
                        message of;
                    }
                }
                occurrence CruiseControl1 {
                    part vehicle_b :> PartsTree::vehicle_b {
                        port redefines setSpeedPort {
                            event occurrence setSpeedReceived;
                        }
                        part redefines speedSensor {
                            port redefines speedSensorPort {
                                event occurrence sensedSpeedSent;
                            }
                        }
                        part redefines vehicleSoftware {
                            part redefines vehicleController {
                                part redefines cruiseController {
                                    port redefines setSpeedPort {
                                        //analagous to gate: event occurrence bound but may not need this since the port is bound
                                        event occurrence setSpeedReceived = vehicle_b.setSpeedPort.setSpeedReceived;
                                    }
                                    port redefines speedSensorPort {
                                        event occurrence sensedSpeedReceived;
                                    }
                                    port redefines cruiseControlPort {
                                        event occurrence fuelCmdSent;
                                    }
                                }
                            }
                        }
                        part redefines engine {
                            port redefines fuelCmdPort {
                                event occurrence fuelCmdReceived;
                            }
                        }
                        message sendSensedSpeed of SensedSpeed from speedSensor.speedSensorPort.sensedSpeedSent to vehicleSoftware.vehicleController.cruiseController.speedSensorPort.sensedSpeedReceived;
                        message sendFuelCmd of FuelCmd from vehicleSoftware.vehicleController.cruiseController.cruiseControlPort.fuelCmdSent to engine.fuelCmdPort.fuelCmdReceived;
                    }
                }
                occurrence CruiseControl2 {
                    part vehicle_b :> PartsTree::vehicle_b {
                        port redefines setSpeedPort {
                            event occurrence setSpeedReceived;
                        }
                        part redefines speedSensor {
                            port redefines speedSensorPort {
                                .sourceEvent;
                            }
                        }
                        part redefines vehicleSoftware {
                            part redefines vehicleController {
                                part redefines cruiseController {
                                    port redefines setSpeedPort {
                                        //analagous to gate: event occurrence bound but may not need this since the port is bound
                                        event occurrence setSpeedReceived = vehicle_b.setSpeedPort.setSpeedReceived;
                                    }
                                    port redefines speedSensorPort {
                                        event occurrence setSpeedReceived = setSpeedPort.setSpeedReceived;
                                        then event sendSensedSpeed.targetEvent;
                                    }
                                    port redefines cruiseControlPort {
                                        .sourceEvent;
                                    }
                                }
                            }
                        }
                        part redefines engine {
                            port redefines fuelCmdPort {
                                .targetEvent;
                            }
                        }
                        message sendSensedSpeed of SensedSpeed;
                        message sendFuelCmd of FuelCmd;
                    }
                }
            }
            package Requirements {
                public import RequirementDerivation::*;
                public import ModelingMetadata::*;
                // incudes status info
                item marketSurvey;
                dependency from vehicleSpecification to marketSurvey;

                requirement vehicleSpecification {
                    subject vehicle : Vehicle;
                    requirement <'1'> vehicleMassRequirement : MassRequirement {
                        doc /* The total mass of the vehicle shall be less than or equal to the required mass.
                        Assume total mass includes a full tank of gas of 60 kg*/
                        attribute redefines massRequired = 2000 [kg];
                        attribute redefines massActual default = vehicle.dryMass + fuelMassActual;
                        attribute fuelMassActual :> ISQ::mass;
                        attribute fuelMassMax :> ISQ::mass = 60 [kg];
                        assume constraint {
                            = fuelMassActual == fuelMassMax;
                        }
                    }

                    allocate vehicleMassRequirement to PartsTree::vehicle_b.mass;

                    requirement <'2'> vehicleFuelEconomyRequirements {
                        doc /* fuel economy requirements group */
                        attribute assumedCargoMass :> ISQ::mass;
                        requirement <'2_1'> cityFuelEconomyRequirement : FuelEconomyRequirement {
                             redefines requiredFuelEconomy = 10 [km / L];
                            assume constraint {
                                = assumedCargoMass <= 500[kg];
                            }
                        }
                        requirement <'2_2'> highwayFuelEconomyRequirement : FuelEconomyRequirement {
                             redefines requiredFuelEconomy = 12.75 [km / L];
                            assume constraint {
                                = assumedCargoMass <= 500[kg];
                            }

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
                    subject engine1 : Engine;
                    requirement <'1'> engineMassRequirement : MassRequirement {
                        doc /* The total mass of the engine shall be less than or equal to the required mass.*/
                        attribute redefines massRequired = 200 [kg];
                        attribute redefines massActual = engine1.mass;
                    }
                    requirement torqueGenerationRequirement : TorqueGenerationRequirement {
                        subject generateTorque default = engine1.generateTorque;
                    }

                    requirement drivePowerOutputRequirement : DrivePowerOutputRequirement {
                        port torqueOutPort {
                            out torque : Torque;
                        }
                    }
                }
                // the engine mass requirement is derived from the vehicle mass requirement
                #derivation
                connection {
                    end #original ::> vehicleSpecification.vehicleMassRequirement;
                    end #derive ::> engineSpecification.engineMassRequirement;
                }
            }
        }
        package Engine4Cyl_Variant {
            public import ModelingMetadata::*;
            // incudes refinement
            part engine : Engine {
                part cylinders : Cylinder [4..8] ordered;
            }
            part engine4Cyl :> engine {
                part redefines cylinders [4];
                part cylinder1 subsets cylinders [1];
                part cylinder2 subsets cylinders [1];
                part cylinder3 subsets cylinders [1];
                part cylinder4 subsets cylinders [1];
            }
            dependency from engine4Cyl to VehicleConfiguration_b::PartsTree::vehicle_b::engine;
        }
        package WheelHubAssemblies {
            // alternative 1 - w/o explicit nesxted interfaces
            part wheelHubAssy1 {
                part wheel1 : Wheel {
                    port :>> lugNutCompositePort : LugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5];
                    }
                }
                part hub1 : Hub {
                    port :>> shankCompositePort : ShankCompositePort {
                        port shankPort :>> shankPort [5];
                    }
                }
                interface wheelHubInterface : WheelHubInterface connect [1] wheel1.lugNutCompositePort to [1] hub1.shankCompositePort;
            }
            // alternative 2 - w multiple nesxted interfaces
            part wheelHubAssy2 {
                part wheel1 : Wheel {
                    port :>> lugNutCompositePort : LugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5];
                    }
                }
                part hub1 : Hub {
                    port :>> shankCompositePort : ShankCompositePort {
                        port shankPort :>> shankPort [5];
                    }
                }
                interface wheelHubInterface : WheelHubInterface connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort {
                    interface wheelFastenerInterface1 :> wheelFastenerInterface connect [5] lugNutPort ::> lugNutCompositePort.lugNutPort to [5] shankPort ::> shankCompositePort.shankPort;
                }
            }
            // alternative 3 - w explicit nesxted interfaces
            part wheelHubAssy3 {
                part wheel1 : Wheel {
                    port lugNutCompositePort :>> lugNutCompositePort {
                        port lugNutPort :>> lugNutPort [5] {
                            attribute :>> threadDia = 14 [mm];
                            attribute :>> threadPitch = 1.5 [mm];
                        }
                        port lugNutPort1 :> lugNutPort [1];
                        port lugNutPort2 :> lugNutPort [1];
                        port lugNutPort3 :> lugNutPort [1];
                    }
                }
                part hub1 : Hub {
                    port shankCompositePort :>> shankCompositePort {
                        port shankPort :>> shankPort [5] {
                            attribute :>> threadDia = 14 [mm];
                            attribute :>> threadPitch = 1.5 [mm];
                            attribute :>> shaftLength = 70 [mm];
                        }
                        port shankPort1 :> shankPort [1];
                        port shankPort2 :> shankPort [1];
                        port shankPort3 :> shankPort [1];
                    }
                }
                interface wheelHubInterface : WheelHubInterface connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort {
                    interface wheelFastenerInterface1 :> wheelFastenerInterface connect lugNutPort ::> lugNutCompositePort.lugNutPort1 to shankPort ::> shankCompositePort.shankPort1 {
                        attribute :>> maxTorque = 90 * 1.356 [N*m];
                    }
                    interface wheelFastenerInterface2 :> wheelFastenerInterface connect lugNutPort ::> lugNutCompositePort.lugNutPort2 to shankPort ::> shankCompositePort.shankPort2 {
                        attribute :>> maxTorque = 90 * 1.356 [N*m];
                    }
                    interface wheelFastenerInterface3 :> wheelFastenerInterface connect lugNutPort ::> lugNutCompositePort.lugNutPort3 to shankPort ::> shankCompositePort.shankPort3 {
                        attribute :>> maxTorque = 90 * 1.356 [N*m];
                    }
                }
            }
        }
    }
    package VehicleAnalysis {
        public import RiskMetadata::*;
        public import RiskLevelEnum::*;
        // recursive public import uses double asterisk **
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        package FuelEconomyAnalysisModel {
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
                attribute wayPoint [1..*] {
                    attribute elapseTime :> ISQ::time [1];
                    attribute position :> ISQ::distance [1];
                }
            }

            calc def FuelConsumption {
                in bestFuelConsumption : Real;
                in idlingFuelConsumption : Real;
                in tpd_avg :> timePerDistance;
                attribute f = bestFuelConsumption + idlingFuelConsumption * tpd_avg;
                return dpv :> distancePerVolume = 1/f;
            }

            calc def AverageTravelTimePerDistance {
                in scenario : Scenario;
                return tpd_avg:>timePerDistance;
            }
            calc def TraveledDistance {
                in scenario : Scenario;
                return distance:> length;
            }
            calc def IdlingFuelConsumptionPerTime {
                in engine : Engine;
                attribute idlingFuelConsumptionPerDisplacement : Real = 0.5;
                return f_a : Real = engine.displacement * idlingFuelConsumptionPerDisplacement;
            }

            attribute specificGravityOfGasoline : Real = 0.76;
            calc def BestFuelConsumptionPerDistance {
                in mass : MassValue;
                in bsfc : Real;
                in tpd_avg :> timePerDistance;
                in distance :> length;
                attribute required_power_avg :> ISQ::power;
                constraint {
                    = required_power_avg == 1 / 2 * mass * tpd_avg ** (-3) / distance;
                }
                return f_b : Real = bsfc * specificGravityOfGasoline * required_power_avg * tpd_avg;
            }

            calc def ComputeBSFC {
                in engine : Engine;
                return : Real;
            }

            analysis fuelEconomyAnalysis {
                subject = vehicle_b;

                objective fuelEconomyAnalysisObjective {
                    doc /*estimate the vehicle fuel economy*/
                    .vehicleFuelEconomyRequirements;
                }

                in attribute scenario : Scenario;
                // define a series of waypoints

                attribute distance = TraveledDistance(scenario);
                attribute tpd_avg = AverageTravelTimePerDistance(scenario);
                attribute bsfc = ComputeBSFC(vehicle_b.engine);
                attribute f_a = BestFuelConsumptionPerDistance(vehicle_b.mass, bsfc, tpd_avg, distance);
                attribute f_b = IdlingFuelConsumptionPerTime(vehicle_b.engine);

                return attribute calculatedFuelEconomy:>distancePerVolume=FuelConsumption(f_a, f_b, tpd_avg);
            }
        }
        package ElectricalPowerAnalysis { }
        package ReliabilityAnalyis { }
        package VehicleTradeOffAnalysis {
            /* The following example provides the rationale for selecting the engine4cyl. 
            The rationale and risk are contained in a metadata library. */

            @Rationale about engineTradeOffAnalysis::vehicle_b_engine4cyl {
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
                in engineMass :> ISQ::mass;
                in enginePower :> ISQ::power;
                in engineFuelEfficiency : Real;
                in engineCost : Real;
                return eval:Real;
            }
            calc def EngineEvaluation_4cyl {
                in engineMass :> ISQ::mass;
                in enginePower :> ISQ::power;
                in engineFuelEfficiency : Real;
                in engineCost : Real;
                return eval:Real;
            }
            calc def EngineEvaluation_6cyl {
                in engineMass :> ISQ::mass;
                in enginePower :> ISQ::power;
                in engineFuelEfficiency : Real;
                in engineCost : Real;
                return eval:Real;
            }
            analysis engineTradeOffAnalysis : TradeStudy {
                subject vehicleAlternatives :> vehicle_b [2];

                part vehicle_b_engine4cyl :> vehicleAlternatives {
                    part engine redefines engine {
                        part cylinders :>> cylinders [4];
                        attribute mass redefines mass = 180 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 180 [W];
                        attribute fuelEfficiency redefines fuelEfficiency = .6;
                        attribute cost redefines cost = 1000;
                    }
                }
                part vehicle_b_engine6cyl :> vehicleAlternatives {
                    part engine redefines engine {
                        part cylinders redefines cylinders [6];
                        attribute mass redefines mass = 220 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 220 [W];
                        attribute fuelEfficiency redefines fuelEfficiency = .5;
                        attribute cost redefines cost = 1500;
                    }
                }

                objective : MaximizeObjective;
                /*Select vehicle alternative with the engine whose evaluation function returns the max value*/

                calc :> evaluationFunction {
                    in part vehicle :> vehicle_b_engine4cyl;
                    return attribute eval:Real=EngineEvaluation_4cyl (vehicle.engine.mass, vehicle.engine.peakHorsePower, vehicle.engine.fuelEfficiency, vehicle.engine.cost);
                }
                calc :> evaluationFunction {
                    in part vehicle :> vehicle_b_engine6cyl;
                    return attribute eval:Real=EngineEvaluation_6cyl (vehicle.engine.mass, vehicle.engine.peakHorsePower, vehicle.engine.fuelEfficiency, vehicle.engine.cost);
                }
                return part selectedVehicle:>vehicle_b;
            }
        }
    }
    package VehicleVerification {
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        public import VerificationCaseDefinitions::*;
        public import VerificationCases1::*;
        // the following is a model library which contains VerdictKind
        public import VerificationCases::*;
        public import VerificationSystem::*;
        package VerificationCaseDefinitions {
            verification def MassTest;
            verification def AccelerationTest;
            verification def ReliabilityTest;
        }
        package VerificationCases1 {
            verification massTests : MassTest {
                subject vehicle_uut :> vehicle_b;
                actor vehicleVerificationSubSystem_1 = verificationContext.massVerificationSystem;
                objective {
                    .vehicleMassRequirement{
                        redefines massActual=weighVehicle.massMeasured;
                    }
                }
                // method kinds are test, demo, analyze, should also include inspection, similarity
                @VerificationMethod {
                    kind = (VerificationMethodKind::test, VerificationMethodKind::analyze);
                }
                action weighVehicle {
                    out massMeasured :> ISQ::mass;
                }
                then action evaluatePassFail {
                    in massMeasured:>ISQ::mass;
                    out verdict = PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut));
                }
                flow from weighVehicle.massMeasured to evaluatePassFail.massMeasured;
                return :>> verdict = evaluatePassFail.verdict;
            }
        }
        package VerificationSystem {
            part verificationContext {
                perform :>> massTests;
                part vehicle_UnitUnderTest :> vehicle_b;
                part massVerificationSystem {
                    part scale {
                        perform :>> massTests.weighVehicle;
                    }
                    part operator {
                        perform :>> massTests.evaluatePassFail;
                    }
                }
            }
        }
    }
    package VehicleIndividuals {
        individual a : VehicleRoadContext_1 {
            timeslice t0_t2_a {
                snapshot t0_a {
                    attribute t0 redefines time = 0 [s];
                    snapshot t0_r : Road_1 {
                        :>> Road::incline = 0;
                        :>> Road::friction = .1;
                    }
                    snapshot t0_v : Vehicle_1 {
                        :>> Vehicle::position = 0 [m];
                        :>> Vehicle::velocity = 0 [m];
                        :>> Vehicle::acceleration = 1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t0_fa : FrontAxleAssembly_1 {
                            snapshot t0_leftFront : Wheel_1;
                            snapshot t0_rightFront : Wheel_2;
                        }
                    }
                }
                snapshot t1_a {
                    attribute t1 redefines time = 1 [s];
                    snapshot t1_r : Road_1 {
                        :>> Road::incline = 0;
                        :>> Road::friction = .1;
                    }
                    snapshot t1_v : Vehicle_1 {
                        :>> Vehicle::position = .98 [m];
                        :>> Vehicle::velocity = 1.96 [m/s];
                        :>> Vehicle::acceleration = 1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t1_fa : FrontAxleAssembly_1 {
                            snapshot t1_leftFront : Wheel_1;
                            snapshot t1_rightFront : Wheel_2;
                        }
                    }
                }
                snapshot t2_a {
                    attribute t2 redefines time = 2 [s];
                    snapshot t2_r : Road_1 {
                        :>> Road::incline = 0;
                        :>> Road::friction = .1;
                    }
                    snapshot t2_v : Vehicle_1 {
                        :>> Vehicle::position = 3.92 [m];
                        :>> Vehicle::velocity = 3.92 [m/s];
                        :>> Vehicle::acceleration = 1.96 [m/s**2];
                        // .2 g where 1 g = 9.8 meters/sec^2
                        snapshot t2_fa : FrontAxleAssembly_1 {
                            snapshot t2_leftFront : Wheel_1;
                            snapshot t2_rightFront : Wheel_2;
                        }
                    }
                }
            }
        }
    }
    package MissionContext {
        /* Define mission context with mission use cases for vehicle_b */
        public import VehicleConfigurations::VehicleConfiguration_b::**;
        public import ParametersOfInterestMetadata::moe;
        public import TransportPassengerScenario::*;
        package ContextDefinitions {
            part def MissionContext :> GenericContext::Context;
            part def Road;
            part def Driver {
                port handPort : HandPort { }
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
            use case def TransportPassenger {
                objective TransportObjective {
                    doc /*deliver passenger to destination safely, comfortably, and within acceptable time*/
                    require constraint transportRequirements;
                }
                subject vehicle : Vehicle;
                actor environment;
                actor road;
                actor driver;
                actor passenger [0..4];
                include use case getInVehicle_a :> getInVehicle [1..5];
                include use case getOutOfVehicle_a :> getOutOfVehicle [1..5];
            }

            use case getInVehicle : GetInVehicle {
                action unlockDoor_in[0..1];
                then action openDoor_in;
                then action enterVehicle;
                then action closeDoor_in;
            }
            use case def GetInVehicle {
                subject vehicle : Vehicle;
                actor driver [0..1];
                actor passenger [0..1];
                assert constraint {
                    = driver != null xor passenger != null;
                }
            }

            use case getOutOfVehicle : GetOutOfVehicle {
                action openDoor_out;
                then action exitVehicle;
                then action closeDoor_out;
                then action lockDoor_out;
            }
            use case def GetOutOfVehicle {
                subject vehicle : Vehicle;
                actor driver [0..1];
                actor passenger [0..1];
                assert constraint {
                    = driver != null xor passenger != null;
                }
            }
        }
        package TransportPassengerScenario {
            public import ContextDefinitions::TransportPassenger;

            // this version uses nesting vs fork and join for concurrent actions
            use case transportPassenger : TransportPassenger {
                first start;
                then action a{
                    action driverGetInVehicle subsets getInVehicle_a[1];
                    action passenger1GetInVehicle subsets getInVehicle_a[1];
                }
                then action trigger
                accept ignitionCmd:IgnitionCmd;
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
            use case transportPassenger_1 : TransportPassenger {
                // declare actions
                action driverGetInVehicle subsets getInVehicle_a [1];
                action passenger1GetInVehicle subsets getInVehicle_a [1];
                action driverGetOutOfVehicle subsets getOutOfVehicle_a [1];
                action passenger1GetOutOfVehicle subsets getOutOfVehicle_a [1];
                action driveVehicleToDestination;
                action providePower;
                item def VehicleOnSignal;
                join join1;
                join join2;
                join join3;
                action trigger;
                accept ignitionCmd:IgnitionCmd;

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

        part missionContext : ContextDefinitions::MissionContext {
            #moe attribute transportTime :> ISQ::time;
            perform :>> transportPassenger;
            // bind parts to actors of use case
            part road : ContextDefinitions::Road = transportPassenger.road;
            part driver : ContextDefinitions::Driver = transportPassenger.driver {
                perform :>> transportPassenger.a.driverGetInVehicle.unlockDoor_in;
                perform :>> transportPassenger.a.driverGetInVehicle.openDoor_in;
                perform :>> transportPassenger.a.driverGetInVehicle.enterVehicle;
                perform :>> transportPassenger.a.driverGetInVehicle.closeDoor_in;
                perform :>> transportPassenger.c.driverGetOutOfVehicle.openDoor_out;
                perform :>> transportPassenger.c.driverGetOutOfVehicle.exitVehicle;
                perform :>> transportPassenger.c.driverGetOutOfVehicle.closeDoor_out;
                perform :>> transportPassenger.c.driverGetOutOfVehicle.lockDoor_out;
                perform :>> transportPassenger.b.driveVehicleToDestination;
            }
            part passenger1 : ContextDefinitions::Passenger = transportPassenger.passenger {
                perform :>> transportPassenger.a.passenger1GetInVehicle.unlockDoor_in;
                perform :>> transportPassenger.a.passenger1GetInVehicle.openDoor_in;
                perform :>> transportPassenger.a.passenger1GetInVehicle.enterVehicle;
                perform :>> transportPassenger.a.passenger1GetInVehicle.closeDoor_in;
                perform :>> transportPassenger.c.passenger1GetOutOfVehicle.openDoor_out;
                perform :>> transportPassenger.c.passenger1GetOutOfVehicle.exitVehicle;
                perform :>> transportPassenger.c.passenger1GetOutOfVehicle.closeDoor_out;
                perform :>> transportPassenger.c.passenger1GetOutOfVehicle.lockDoor_out;
            }
            part vehicle_b_1 :> vehicle_b = transportPassenger.vehicle {
                attribute :>> position3dVector = (0,0,0) [spatialCF];
                perform :>> transportPassenger.b.providePower;
                 redefines providePower;
                perform :>> transportPassenger.trigger;
            }
            connect driver.handPort to vehicle_b_1.ignitionCmdPort;
            connect road to vehicle_b_1.vehicleToRoadPort;
        }
    }
    package VehicleSuperSetModel {
        /* all of vehicleFamily is included in the superset model to enable subsetting a specific vehicle configuration*/
        package VariationPointDefinitions {
            variation part def TransmissionChoices :> Transmission {
                variant part transmissionAutomatic:TransmissionAutomatic;
                variant part transmissionManual:TransmissionManual;
            }
        }
        package VehiclePartsTree {
            public import VariationPointDefinitions::*;
            abstract part vehicleFamily {
                // variation with nested variation
                variation part engine : Engine {
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
                part transmissionChoices : TransmissionChoices;
                // optional variation point
                part sunroof : Sunroof [0..1];
                // selection constraint
                assert constraint selectionConstraint {
                    = (engine == engine::engine4Cyl and transmissionChoices == TransmissionChoices::transmissionManual) xor (engine == engine::engine6Cyl and transmissionChoices == TransmissionChoices::transmissionAutomatic);
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
    package Views_Viewpoints {
        package ViewpointDefinitions {
            viewpoint def BehaviorViewpoint;
            viewpoint def SafetyViewpoint {
                frame vs : VehicleSafety;
            }
            part def SafetyEngineer;
            concern def VehicleSafety {
                doc /* identify system safety features */
                subject;
                stakeholder se : SafetyEngineer;
            }
        }
        package ViewDefinitions {
            //public import Views to access rendering method library 
            public import Views::*;
            view def TreeView {
                render asTreeDiagram;
            }
            view def NestedView;
            view def RelationshipView;
            view def TableView;
            view def PartsTreeView :> TreeView {
                filter @SysML::PartUsage;
            }
            view def PartsInterconnection :> NestedView;
        }
        package VehicleViews {
            public import ViewpointDefinitions::*;
            public import ViewDefinitions::*;
            public import VehicleConfigurations::VehicleConfiguration_b::*;
            view vehiclePartsTree_Safety : PartsTreeView {
                satisfy sv : SafetyViewpoint;
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
(model
  (namespace
    (package 'SimpleVehicleModel'
      (namespace_import public -> 'SimpleVehicleModel::Definitions'[package])
      (namespace_import public -> 'ISQ'[unresolved])
      (package 'Definitions'
        (namespace_import public -> 'SimpleVehicleModel::Definitions::PartDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::PortDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::ItemDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::SignalDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::InterfaceDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::AllocationDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::ActionDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::StateDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::RequirementDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::AttributeDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::IndividualDefinitions'[package])
        (membership_import public recursive -> 'SimpleVehicleModel::Definitions::MetadataDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions'[package])
        (package 'PartDefinitions'
          (part_def 'Vehicle'
            (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
            (attribute_usage composite 'dryMass' :> 'ISQ::mass'[unresolved])
            (attribute_usage composite 'cargoMass' :> 'ISQ::mass'[unresolved])
            (attribute_usage composite 'position' :> 'ISQ::length'[unresolved])
            (attribute_usage composite 'velocity' :> 'ISQ::speed'[unresolved])
            (attribute_usage composite 'acceleration' :> 'ISQ::acceleration'[unresolved])
            (attribute_usage composite 'electricalPower' :> 'ISQ::power'[unresolved])
            (attribute_usage composite 'Tmax' :> 'ISQ::temperature'[unresolved])
            (attribute_usage composite 'maintenanceTime' : 'Time::DateTime'[unresolved])
            (attribute_usage composite 'brakePedalDepressed' : 'Boolean'[unresolved])
            (port_usage composite 'ignitionCmdPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort'[port_def])
            (port_usage composite 'pwrCmdPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort'[port_def])
            (port_usage composite 'vehicleToRoadPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort'[port_def])
            (port_usage composite 'statusPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::StatusPort'[port_def])
            (perform_action_usage 'providePower')
            (perform_action_usage 'provideBraking')
            (perform_action_usage 'controlDirection')
            (perform_action_usage 'performSelfTest')
            (perform_action_usage 'applyParkingBrake')
            (perform_action_usage 'senseTemperature')
            (state_usage parallel composite 'vehicleStates'
              (reference_usage reference 'controller' : 'SimpleVehicleModel::Definitions::PartDefinitions::VehicleController'[part_def])
              (state_usage composite 'operatingStates'
                (state_subaction_membership 'entry'
                  (action_usage 'initial'))
                (state_usage composite 'off')
                (state_usage composite 'starting')
                (state_usage composite 'on'
                  (state_subaction_membership 'entry'
                    (action_usage 'performSelfTest'))
                  (state_subaction_membership 'do'
                    (action_usage 'providePower'))
                  (state_subaction_membership 'exit'
                    (action_usage 'applyParkingBrake'))
                  (constraint_usage composite
                    (result_expr_membership)))
                (transition_usage)
                (transition_usage 'off_To_starting')
                (transition_usage 'starting_To_on')
                (transition_usage 'on_To_off'))
              (state_usage composite 'healthStates'
                (state_subaction_membership 'entry'
                  (action_usage 'initial'))
                (state_subaction_membership 'do'
                  (action_usage 'senseTemperature'
                    (reference_usage out reference 'temp')))
                (state_usage composite 'normal')
                (state_usage composite 'maintenance')
                (state_usage composite 'degraded')
                (transition_usage)
                (transition_usage 'normal_To_maintenance')
                (transition_usage 'normal_To_degraded')
                (transition_usage 'maintenance_To_normal')
                (transition_usage 'degraded_To_normal'))))
          (part_def 'Engine'
            (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
            (attribute_usage composite 'peakHorsePower' :> 'ISQ::power'[unresolved])
            (attribute_usage composite 'fuelEfficiency' : 'Real'[unresolved])
            (attribute_usage composite 'cost' : 'Real'[unresolved])
            (attribute_usage composite 'displacement' :> 'ISQ::volume'[unresolved])
            (port_usage composite 'engineControlPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ControlPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::ControlPort'[port_def])
            (port_usage composite 'fuelInPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def])
            (port_usage composite 'fuelCmdPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort'[port_def])
            (port_usage composite 'drivePwrPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort'[port_def])
            (port_usage composite 'ignitionCmdPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort'[port_def])
            (port_usage composite 'flyWheelPort')
            (perform_action_usage 'generateTorque')
            (state_usage composite 'engineStates'
              (state_usage composite 'off')
              (state_usage composite 'starting')
              (state_usage composite 'on'
                (state_subaction_membership 'do'
                  (action_usage 'generateTorque')))))
          (part_def 'StarterMotor'
            (port_usage composite 'gearPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::GearPort'[port_def]))
          (part_def 'Cylinder')
          (part_def 'Transmission'
            (attribute_usage composite 'gearRatio' : 'Real'[unresolved])
            (port_usage composite 'clutchPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort'[port_def])
            (state_usage composite 'transmissionStates'))
          (part_def 'Driveshaft')
          (part_def 'AxleAssembly')
          (part_def 'Axle'
            (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
          (part_def 'FrontAxle' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Axle'[part_def]
            (attribute_usage composite 'steeringAngle' :> 'ISQ::angularMeasure'[unresolved]))
          (part_def 'HalfAxle'
            (port_usage composite 'shankCompositePort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort'[port_def]))
          (part_def 'Differential')
          (part_def 'Wheel'
            (attribute_usage composite 'diameter' : 'LengthValue'[unresolved])
            (port_usage composite 'lugNutCompositePort' : 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort'[port_def]))
          (part_def 'Hub'
            (port_usage composite 'shankCompositePort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort'[port_def]))
          (part_def abstract 'Software')
          (part_def 'VehicleSoftware' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Software'[part_def])
          (part_def 'VehicleController' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Software'[part_def]
            (port_usage composite 'controlPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ControlPort'[port_def])
            (state_usage parallel composite 'controllerStates'
              (state_usage composite 'operatingStates'
                (state_subaction_membership 'entry'
                  (action_usage 'initial'))
                (state_usage composite 'off')
                (state_usage composite 'on')
                (transition_usage)
                (transition_usage)
                (transition_usage))))
          (part_def 'CruiseController' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Software'[part_def]
            (port_usage composite 'setSpeedPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort'[port_def])
            (port_usage composite 'speedSensorPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort'[port_def])
            (port_usage composite 'cruiseControlPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort'[port_def])
            (state_usage composite 'cruiseControllerStates'))
          (part_def 'SpeedSensor'
            (port_usage composite 'speedSensorPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort'[port_def]))
          (part_def 'FuelTank'
            (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
            (item_usage reference 'fuel' : 'SimpleVehicleModel::Definitions::ItemDefinitions::Fuel'[item_def]
              (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::ItemDefinitions::Fuel::fuelMass'[attribute_usage]))
            (attribute_usage composite 'fuelKind' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind'[enum_def])
            (attribute_usage composite 'fuelMassMax' :> 'ISQ::mass'[unresolved])
            (assert_constraint_usage 'fuelConstraint'
              (result_expr_membership))
            (port_usage composite 'fuelOutPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def])
            (port_usage composite 'fuelInPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def]))
          (part_def 'BodyAssy')
          (part_def 'Body'
            (attribute_usage composite 'color' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Colors'[enum_def]))
          (part_def 'Thermostat')
          (part_def 'WaterHose')
          (part_def 'Road'
            (attribute_usage composite 'incline' : 'Real'[unresolved])
            (attribute_usage composite 'friction' : 'Real'[unresolved]))
          (part_def 'Engine4Cyl')
          (part_def 'Engine6Cyl')
          (part_def 'TransmissionChoices')
          (part_def 'TransmissionAutomatic')
          (part_def 'TransmissionManual')
          (part_def 'Sunroof')
          (part_def 'ElectricalGenerator')
          (part_def 'TorqueGenerator')
          (part_def 'SteeringSubsystem')
          (part_def 'BrakingSubsystem'))
        (package 'PortDefinitions'
          (port_def 'IgnitionCmdPort'
            (item_usage in 'ignitionCmd' : 'SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd'[item_def]))
          (port_def 'StatusPort')
          (port_def 'GearPort')
          (port_def 'PwrCmdPort'
            (item_usage in 'pwrCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd'[item_def]))
          (port_def 'FuelCmdPort' :> 'SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort'[port_def]
            (item_usage in 'fuelCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd'[item_def] :>> 'SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort::pwrCmd'[item_usage]))
          (port_def 'FuelPort'
            (item_usage out 'fuel' : 'SimpleVehicleModel::Definitions::ItemDefinitions::Fuel'[item_def]))
          (port_def 'DrivePwrPort'
            (reference_usage out reference 'torque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]))
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
            (port_usage composite 'lugNutPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort'[port_def]
              (multiplicity_range [*])))
          (port_def 'ShankCompositePort'
            (port_usage composite 'shankPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankPort'[port_def]
              (multiplicity_range [*])))
          (port_def 'LugNutPort'
            (attribute_usage composite 'threadDia')
            (attribute_usage composite 'threadPitch'))
          (port_def 'ShankPort'
            (attribute_usage composite 'threadDia')
            (attribute_usage composite 'threadPitch')
            (attribute_usage composite 'shaftLength'))
          (port_def 'VehicleToRoadPort')
          (port_def 'ControlPort')
          (port_def 'CruiseControlPort' :> 'SimpleVehicleModel::Definitions::PortDefinitions::ControlPort'[port_def])
          (port_def 'SpeedSensorPort')
          (port_def 'SetSpeedPort')
          (port_def 'DriverCmdPort'
            (item_usage out 'driverCmd' : 'SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd'[item_def]
              (multiplicity_range [*])))
          (port_def 'HandPort' :> 'SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort'[port_def]
            (item_usage out 'ignitionCmd' : 'SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd'[item_def] :> 'driverCmd'[unresolved] :>> 'SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort::driverCmd'[item_usage][implied])
            (item_usage out 'pwrCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd'[item_def] :> 'driverCmd'[unresolved])))
        (package 'ItemDefinitions'
          (item_def 'PwrCmd'
            (attribute_usage composite 'throttleLevel' : 'Real'[unresolved]))
          (item_def 'FuelCmd' :> 'SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd'[item_def])
          (item_def 'Fuel'
            (attribute_usage composite 'fuelMass' :> 'ISQ::mass'[unresolved]))
          (item_def 'SensedSpeed'
            (attribute_usage composite 'speed' :> 'ISQ::speed'[unresolved])))
        (package 'SignalDefinitions'
          (item_def 'Cmd')
          (item_def 'DriverCmd')
          (item_def 'IgnitionCmd' :> 'SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd'[item_def]
            (attribute_usage composite 'ignitionOnOff' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff'[enum_def]))
          (item_def 'EngineStatus')
          (attribute_def 'VehicleStartSignal')
          (attribute_def 'VehicleOnSignal')
          (attribute_def 'VehicleOffSignal')
          (attribute_def 'StartSignal')
          (attribute_def 'OffSignal')
          (attribute_def 'OverTemp')
          (attribute_def 'ReturnToNormal')
          (attribute_def 'SetSpeed' :> 'Real'[unresolved]))
        (package 'InterfaceDefinitions'
          (interface_def 'EngineToTransmissionInterface'
            (port_usage end 'p1' : 'SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort'[port_def])
            (port_usage end 'p2' : 'SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort'[port_def])
            (flow_usage composite 'p1'))
          (interface_def 'FuelInterface'
            (port_usage end 'fuelOutPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def])
            (port_usage end 'fuelInPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelPort'[port_def])
            (flow_usage composite 'of'))
          (interface_def 'WheelFastenerInterface'
            (port_usage end 'lugNutPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort'[port_def])
            (port_usage end 'shankPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankPort'[port_def])
            (attribute_usage composite 'maxTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member])
            (constraint_usage composite
              (result_expr_membership)))
          (interface_def 'WheelHubInterface'
            (port_usage end 'lugNutCompositePort' : 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort'[port_def])
            (port_usage end 'shankCompositePort' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort'[port_def])
            (interface_usage composite 'wheelFastenerInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface'[interface_def]
              (multiplicity_range [5])
              (connector_end 'lugNutCompositePort.lugNutPort')
              (connector_end 'shankCompositePort.shankPort'))))
        (package 'AllocationDefinitions'
          (allocation_def 'LogicalToPhysical'
            (port_usage end 'logicalEnd')
            (port_usage end 'physicalEnd')))
        (package 'ActionDefinitions'
          (action_def 'ProvidePower'
            (item_usage in 'pwrCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd'[item_def])
            (reference_usage out reference 'wheelToRoadTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]
              (multiplicity_range [2])))
          (action_def 'GenerateTorque'
            (item_usage in 'fuelCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd'[item_def])
            (reference_usage out reference 'engineTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]))
          (action_def 'AmplifyTorque'
            (reference_usage in reference 'engineTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member])
            (reference_usage out reference 'transmissionTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]))
          (action_def 'TransferTorque'
            (reference_usage in reference 'transmissionTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member])
            (reference_usage out reference 'driveshaftTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]))
          (action_def 'DistributeTorque'
            (reference_usage in reference 'driveshaftTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member])
            (reference_usage out reference 'wheelToRoadTorque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]
              (multiplicity_range [2])))
          (action_def 'PerformSelfTest')
          (action_def 'ApplyParkingBrake')
          (action_def 'SenseTemperature'
            (reference_usage out reference 'temp' : 'ISQ::TemperatureValue'[unresolved])))
        (package 'StateDefinitions'
          (state_def 'VehicleStates')
          (state_def 'ControllerStates')
          (state_def 'CruiseControllerStates'))
        (package 'RequirementDefinitions'
          (requirement_def 'MassRequirement'
            (documentation)
            (attribute_usage composite 'massRequired' :> 'ISQ::mass'[unresolved])
            (attribute_usage composite 'massActual' :> 'ISQ::mass'[unresolved])
            (require_constraint_usage composite
              (result_expr_membership)))
          (requirement_def 'ReliabilityRequirement'
            (documentation)
            (attribute_usage composite 'reliabilityRequired' : 'Real'[unresolved])
            (attribute_usage composite 'reliabilityActual' : 'Real'[unresolved])
            (require_constraint_usage composite
              (result_expr_membership)))
          (requirement_def 'TorqueGenerationRequirement'
            (documentation)
            (subject_membership in 'generateTorque' : 'SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque'[action_def]))
          (requirement_def 'DrivePowerOutputRequirement'
            (documentation))
          (requirement_def 'FuelEconomyRequirement'
            (documentation)
            (attribute_usage composite 'actualFuelEconomy' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::distancePerVolume'[feature_def])
            (attribute_usage composite 'requiredFuelEconomy' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::distancePerVolume'[feature_def])
            (require_constraint_usage composite
              (result_expr_membership))))
        (package 'AttributeDefinitions'
          (namespace_import public -> 'ScalarValues'[unresolved])
          (namespace_import public -> 'Quantities'[unresolved])
          (membership_import public -> 'MeasurementReferences::DerivedUnit'[unresolved])
          (membership_import public -> 'SIPrefixes::kilo'[unresolved])
          (namespace_import public -> 'NumericalFunctions'[unresolved])
          (namespace_import public -> 'SI'[unresolved])
          (namespace_import public -> 'USCustomaryUnits'[unresolved])
          (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved])
          (enum_def 'Colors'
            (enum_usage composite 'black')
            (enum_usage composite 'grey')
            (enum_usage composite 'red'))
          (enum_def 'DiameterChoices' :> 'ISQ::LengthValue'[unresolved]
            (not_implemented 'malformed')
            (not_implemented 'malformed')
            (not_implemented 'malformed'))
          (attribute_usage 'cylinderDiameter' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices'[enum_def]
            (feature_value (=)))
          (enum_def 'IgnitionOnOff'
            (enum_usage composite 'on')
            (enum_usage composite 'off'))
          (enum_def 'FuelKind'
            (enum_usage composite 'gas')
            (enum_usage composite 'diesel'))
          (feature_def 'distancePerVolume' :> 'scalarQuantities'[unresolved]
            (feature_value (=)))
          (feature_def 'timePerDistance' :> 'scalarQuantities'[unresolved]
            (feature_value (=)))
          (feature_def 'volumePerDistance' :> 'scalarQuantities'[unresolved]
            (feature_value (=)))
          (feature_def 'volumePerTime' :> 'scalarQuantities'[unresolved]
            (feature_value (=)))
          (feature_def 'kpl' : 'DerivedUnit'[unresolved]
            (feature_value (=)))
          (feature_def 'rpm' : 'DerivedUnit'[unresolved]
            (feature_value (=)))
          (feature_def 'kW' : 'DerivedUnit'[unresolved]
            (feature_value (=))))
        (package 'IndividualDefinitions'
          (occurrence_def individual 'VehicleRoadContext_1' :> 'SimpleVehicleModel::Definitions::GenericContext::Context'[part_def])
          (occurrence_def individual 'Vehicle_1' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def])
          (occurrence_def individual 'FrontAxleAssembly_1' :> 'SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly'[part_def])
          (occurrence_def individual 'FrontAxle_1' :> 'SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle'[part_def])
          (occurrence_def individual 'Wheel_1' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def])
          (occurrence_def individual 'Wheel_2' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def])
          (occurrence_def individual 'RearAxleAssembly_1' :> 'SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly'[part_def])
          (occurrence_def individual 'Road_1' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Road'[part_def]))
        (package 'MetadataDefinitions'
          (namespace_import public -> 'AnalysisTooling'[unresolved])
          (metadata_def 'Safety'
            (attribute_usage composite 'isMandatory' : 'Boolean'[unresolved]))
          (metadata_def 'Security'))
        (package 'KeyWord_MetadataDefinitions'
          (membership_import public -> 'Metaobjects::SemanticMetadata'[unresolved])
          (state_usage 'failureModes'
            (multiplicity_range [*]))
          (metadata_def 'failureMode' :> 'SemanticMetadata'[unresolved]
            (reference_usage reference :>> 'baseType'[unresolved]
              (feature_value (=))))
          (occurrence_usage 'logicalOccurrences'
            (multiplicity_range [*]))
          (metadata_def 'logical' :> 'SemanticMetadata'[unresolved]
            (reference_usage reference :>> 'baseType'[unresolved]
              (feature_value (=))))
          (occurrence_usage 'physicalOccurrences'
            (multiplicity_range [*]))
          (metadata_def 'physical' :> 'SemanticMetadata'[unresolved]
            (reference_usage reference :>> 'baseType'[unresolved]
              (feature_value (=)))))
        (package 'GenericContext'
          (part_def 'Context'
            (attribute_usage composite 'time' : 'TimeValue'[unresolved])
            (attribute_usage composite 'spatialCF' : 'CartesianSpatial3dCoordinateFrame'[unresolved]
              (multiplicity_range [1])
              (reference_usage reference :>> 'mRefs'[unresolved]
                (feature_value (=))))
            (attribute_usage composite 'velocityCF' : 'CartesianVelocity3dCoordinateFrame'[unresolved]
              (multiplicity_range [1])
              (feature_value (=)))
            (attribute_usage composite 'accelarationCF' : 'CartesianAcceleration3dCoordinateFrame'[unresolved]
              (multiplicity_range [1])
              (feature_value (=))))))
      (package 'VehicleLogicalConfiguration'
        (package 'PartsTree'
          (part_usage 'vehicleLogical' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def]
            (part_usage composite 'torqueGenerator' : 'SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator'[part_def]
              (action_usage composite 'generateTorque'))
            (part_usage composite 'electricalGenerator' : 'SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator'[part_def]
              (action_usage composite 'generateElectricity'))
            (part_usage composite 'steeringSystem' : 'SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem'[part_def])
            (part_usage composite 'brakingSubsystem' : 'SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem'[part_def]))))
      (package 'VehicleLogicalToPhysicalAllocation'
        (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree'[package])
        (namespace_import public -> 'SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree'[package])
        (allocation_usage 'vehicleLogicalToPhysicalAllocation' : 'SimpleVehicleModel::Definitions::AllocationDefinitions::LogicalToPhysical'[allocation_def]
          (connector_end 'vehicleLogical')
          (connector_end 'vehicle_b')
          (allocation_usage composite
            (connector_end 'vehicleLogical.torqueGenerator')
            (connector_end 'vehicle_b.engine')
            (allocation_usage composite
              (connector_end 'vehicleLogical.torqueGenerator.generateTorque')
              (connector_end 'vehicle_b.engine.generateTorque')))
          (allocation_usage composite
            (connector_end 'vehicleLogical.electricalGenerator')
            (connector_end 'vehicle_b.engine')
            (allocation_usage composite
              (connector_end 'vehicleLogical.electricalGenerator.generateElectricity')
              (connector_end 'vehicle_b.engine.alternator.generateElectricity')))))
      (package 'VehicleConfigurations'
        (package 'VehicleConfiguration_a'
          (package 'PartsTree'
            (part_usage 'vehicle_a' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def]
              (attribute_usage composite 'mass' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass'[attribute_usage]
                (feature_value (=)))
              (attribute_usage composite 'dryMass' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass'[attribute_usage]
                (feature_value (=)))
              (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass'[attribute_usage]
                (feature_value (=)))
              (attribute_usage composite 'partMasses' :> 'ISQ::mass'[unresolved]
                (multiplicity_range [*]))
              (part_usage composite 'fuelTank' : 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank'[part_def]
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass'[attribute_usage]
                  (feature_value (=)))
                (item_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuel'[item_usage]
                  (attribute_usage composite :>> ''[attribute_usage]
                    (feature_value (=)))))
              (part_usage composite 'frontAxleAssembly' : 'SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly'[part_def]
                (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (part_usage composite 'frontAxle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Axle'[part_def])
                (part_usage composite 'frontWheels' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
                  (multiplicity_range [2])))
              (part_usage composite 'rearAxleAssembly' : 'SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly'[part_def]
                (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (attribute_usage composite 'driveTrainEfficiency' : 'Real'[unresolved]
                  (feature_value (=)))
                (part_usage composite 'rearAxle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Axle'[part_def])
                (part_usage composite 'rearWheels' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
                  (multiplicity_range [2])
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter'[attribute_usage])))))
          (package 'ActionTree')
          (package 'Requirements'))
        (package 'VehicleConfiguration_b'
          (membership_import public -> 'ShapeItems::Box'[unresolved])
          (membership_import public -> 'ParametersOfInterestMetadata::mop'[unresolved])
          (namespace_import public -> 'ModelingMetadata'[unresolved])
          (package 'PartsTree'
            (part_usage 'vehicle_b' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def]
              (attribute_usage composite 'mass' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass'[attribute_usage]
                (feature_value (=)))
              (attribute_usage composite 'dryMass' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass'[attribute_usage]
                (feature_value (=)))
              (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass'[attribute_usage]
                (feature_value (default =)))
              (attribute_usage composite 'partMasses'
                (feature_value (=)))
              (attribute_usage composite 'avgFuelEconomy' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::distancePerVolume'[feature_def])
              (port_usage composite 'fuelCmdPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort'[port_def] :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort'[port_usage]
                (item_usage in 'fuelCmd' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort::pwrCmd'[item_usage]))
              (port_usage composite 'setSpeedPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort'[port_def] ~ 'SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort'[port_def])
              (port_usage composite 'vehicleToRoadPort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort'[port_usage]
                (port_usage composite 'wheelToRoadPort1' : 'SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort'[port_def])
                (port_usage composite 'wheelToRoadPort2' : 'SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort'[port_def]))
              (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower'[action_usage])
              (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::providePower'[perform_action_usage])
              (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest'[action_usage])
              (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::performSelfTest'[perform_action_usage])
              (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake'[action_usage])
              (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::applyParkingBrake'[perform_action_usage])
              (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature'[action_usage])
              (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::senseTemperature'[perform_action_usage])
              (state_usage composite 'vehicleStates' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleStates'[state_usage])
              (item_usage composite :> 'envelopingShapes'[unresolved] : 'Box'[unresolved]
                (multiplicity_range [1])
                (reference_usage reference 'length1' :>> 'length'[unresolved]
                  (feature_value (=)))
                (reference_usage reference 'width1' :>> 'width'[unresolved]
                  (feature_value (=)))
                (reference_usage reference 'height1' :>> 'height'[unresolved]
                  (feature_value (=))))
              (part_usage composite 'fuelTank' : 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank'[part_def]
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass'[attribute_usage]
                  (feature_value (=)))
                (item_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuel'[item_usage]
                  (attribute_usage composite :>> ''[attribute_usage]
                    (feature_value (=))))
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax'[attribute_usage]
                  (feature_value (=))))
              (part_usage composite 'frontAxleAssembly' : 'SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly'[part_def]
                (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (port_usage composite 'shaftPort_d' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d'[port_def])
                (part_usage composite 'frontAxle' : 'SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle'[part_def])
                (part_usage composite 'frontWheels' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
                  (multiplicity_range [2])))
              (part_usage composite 'rearAxleAssembly' : 'SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly'[part_def]
                (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (attribute_usage composite 'driveTrainEfficiency' : 'Real'[unresolved]
                  (feature_value (=)))
                (port_usage composite 'shaftPort_d' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d'[port_def])
                (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque'[action_usage])
                (part_usage composite 'rearWheel1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter'[attribute_usage])
                  (port_usage composite 'wheelToRoadPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort'[port_def])
                  (port_usage composite 'lugNutCompositePort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort'[port_usage]
                    (port_usage composite 'lugNutPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort'[port_usage]
                      (multiplicity_range [5]))))
                (part_usage composite 'rearWheel2' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter'[attribute_usage])
                  (port_usage composite 'wheelToRoadPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort'[port_def])
                  (port_usage composite 'lugNutCompositePort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort'[port_usage]
                    (port_usage composite 'lugNutPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort'[port_usage]
                      (multiplicity_range [5]))))
                (part_usage composite 'differential' : 'SimpleVehicleModel::Definitions::PartDefinitions::Differential'[part_def]
                  (port_usage composite 'shaftPort_d' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d'[port_def])
                  (port_usage composite 'leftDiffPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::DiffPort'[port_def])
                  (port_usage composite 'rightDiffPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::DiffPort'[port_def]))
                (part_usage composite 'rearAxle'
                  (part_usage composite 'leftHalfAxle' : 'SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle'[part_def]
                    (port_usage composite 'leftAxleToDiffPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::AxlePort'[port_def])
                    (port_usage composite 'shankCompositePort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort'[port_usage]
                      (port_usage composite 'shankPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort'[port_usage]
                        (multiplicity_range [5]))))
                  (part_usage composite 'rightHalfAxle' : 'SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle'[part_def]
                    (port_usage composite 'rightAxleToDiffPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::AxlePort'[port_def])
                    (port_usage composite 'shankCompositePort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort'[port_usage]
                      (port_usage composite 'shankPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort'[port_usage]
                        (multiplicity_range [5])))))
                (binding_connector_def
                  (connector_end 'shaftPort_d')
                  (connector_end 'differential.shaftPort_d'))
                (connection_usage composite
                  (connector_end 'differential.leftDiffPort')
                  (connector_end 'rearAxle.leftHalfAxle.leftAxleToDiffPort'))
                (connection_usage composite
                  (connector_end 'differential.rightDiffPort')
                  (connector_end 'rearAxle.rightHalfAxle.rightAxleToDiffPort'))
                (interface_usage composite 'wheelToleftHalAxleInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface'[interface_def]
                  (connector_end 'rearWheel1.lugNutCompositePort')
                  (connector_end 'rearAxle.leftHalfAxle.shankCompositePort'))
                (interface_usage composite 'wheelTorightHalAxleInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface'[interface_def]
                  (connector_end 'rearWheel2.lugNutCompositePort')
                  (connector_end 'rearAxle.rightHalfAxle.shankCompositePort')))
              (part_usage composite 'starterMotor' : 'SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor'[part_def])
              (part_usage composite 'engine' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine'[part_def]
                (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque'[action_usage])
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::generateTorque'[perform_action_usage])
                (part_usage composite 'cylinders' : 'SimpleVehicleModel::Definitions::PartDefinitions::Cylinder'[part_def]
                  (multiplicity_range [4..6]))
                (part_usage composite 'alternator'
                  (action_usage composite 'generateElectricity'))
                (not_implemented 'malformed'))
              (part_usage composite 'transmission' : 'SimpleVehicleModel::Definitions::PartDefinitions::Transmission'[part_def]
                (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (port_usage composite 'shaftPort_a' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a'[port_def])
                (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque'[action_usage]))
              (part_usage composite 'driveshaft' : 'SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft'[part_def]
                (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (port_usage composite 'shaftPort_b' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b'[port_def])
                (port_usage composite 'shaftPort_c' : 'SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c'[port_def])
                (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque'[action_usage]))
              (part_usage composite 'vehicleSoftware' : 'SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware'[part_def]
                (part_usage composite 'vehicleController' : 'SimpleVehicleModel::Definitions::PartDefinitions::VehicleController'[part_def]
                  (state_usage composite 'controllerStates' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controllerStates'[state_usage])
                  (part_usage composite 'cruiseController' : 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController'[part_def])))
              (part_usage composite 'speedSensor' : 'SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor'[part_def])
              (part_usage composite 'bodyAssy' : 'SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy'[part_def]
                (part_usage composite 'body' : 'SimpleVehicleModel::Definitions::PartDefinitions::Body'[part_def]
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Body::color'[attribute_usage]
                    (feature_value (=))))
                (part_usage composite 'bumper'
                  (metadata_usage :> 'Safety'[unresolved]
                    (feature_def 'isMandatory'
                      (feature_value (=)))))
                (part_usage composite 'keylessEntry'
                  (metadata_usage :> 'Security'[unresolved])))
              (part_usage composite 'interior'
                (part_usage composite 'alarm'
                  (metadata_usage :> 'Security'[unresolved]))
                (part_usage composite 'seatBelt'
                  (multiplicity_range [2])
                  (metadata_usage :> 'Safety'[unresolved]
                    (feature_def 'isMandatory'
                      (feature_value (=)))))
                (part_usage composite 'frontSeat'
                  (multiplicity_range [2]))
                (part_usage composite 'driverAirBag'
                  (metadata_usage :> 'Safety'[unresolved]
                    (feature_def 'isMandatory'
                      (feature_value (=))))))
              (binding_connector_def
                (connector_end 'engine.fuelCmdPort')
                (connector_end 'fuelCmdPort'))
              (interface_usage composite 'engineToTransmissionInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::EngineToTransmissionInterface'[interface_def]
                (connector_end 'engine.drivePwrPort')
                (connector_end 'transmission.clutchPort'))
              (interface_usage composite 'fuelInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::FuelInterface'[interface_def]
                (connector_end 'fuelTank.fuelOutPort')
                (connector_end 'engine.fuelInPort'))
              (allocation_usage composite
                (connector_end 'ActionTree::providePower.generateToAmplify')
                (connector_end 'engineToTransmissionInterface'))
              (binding_connector_def
                (connector_end 'engine.ignitionCmdPort')
                (connector_end 'ignitionCmdPort'))
              (connection_usage composite
                (connector_end 'starterMotor.gearPort')
                (connector_end 'engine.flyWheelPort'))
              (connection_usage composite
                (connector_end 'vehicleSoftware.vehicleController.controlPort')
                (connector_end 'engine.engineControlPort'))
              (binding_connector_def
                (connector_end 'vehicle_b.setSpeedPort')
                (connector_end 'vehicleSoftware.vehicleController.cruiseController.setSpeedPort'))
              (connection_usage composite
                (connector_end 'speedSensor.speedSensorPort')
                (connector_end 'vehicleSoftware.vehicleController.cruiseController.speedSensorPort'))
              (binding_connector_def
                (connector_end 'vehicleSoftware.vehicleController.cruiseController.cruiseControlPort')
                (connector_end 'vehicleSoftware.vehicleController.controlPort'))
              (connection_usage composite
                (connector_end 'transmission.shaftPort_a')
                (connector_end 'driveshaft.shaftPort_b'))
              (connection_usage composite
                (connector_end 'driveshaft.shaftPort_c')
                (connector_end 'rearAxleAssembly.shaftPort_d'))
              (binding_connector_def
                (connector_end 'rearAxleAssembly.rearWheel1.wheelToRoadPort')
                (connector_end 'vehicleToRoadPort.wheelToRoadPort1'))
              (binding_connector_def
                (connector_end 'rearAxleAssembly.rearWheel2.wheelToRoadPort')
                (connector_end 'vehicleToRoadPort.wheelToRoadPort2'))
              (not_implemented 'malformed')))
          (package 'ActionTree'
            (action_usage 'providePower' : 'SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower'[action_def]
              (item_usage in 'fuelCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd'[item_def] :>> 'SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd'[item_usage])
              (reference_usage out reference 'wheelToRoadTorque' :>> 'SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::wheelToRoadTorque'[reference_usage]
                (multiplicity_range [2])
                (feature_value (=)))
              (action_usage composite 'generateTorque' : 'SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque'[action_def]
                (not_implemented 'malformed')
                (not_implemented 'malformed'))
              (action_usage composite 'amplifyTorque' : 'SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque'[action_def])
              (action_usage composite 'transferTorque' : 'SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque'[action_def])
              (action_usage composite 'distributeTorque' : 'SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque'[action_def])
              (flow_usage composite 'generateToAmplify'
                (connector_end 'generateTorque.engineTorque')
                (connector_end 'amplifyTorque.engineTorque'))
              (flow_usage composite 'amplifyTorque')
              (flow_usage composite 'transferTorque'))
            (action_usage 'performSelfTest' : 'SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest'[action_def])
            (action_usage 'applyParkingBrake' : 'SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake'[action_def])
            (action_usage 'senseTemperature' : 'SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature'[action_def]))
          (package 'DiscreteInteractions'
            (package 'Sequence'
              (part_def 'Driver'
                (port_usage composite 'p1')
                (port_usage composite 'p2'))
              (part_usage 'part0'
                (perform_action_usage 'startVehicle'
                  (action_usage 'turnVehicleOn')
                  (send_action_usage
                    (reference_usage in reference 'ignitionCmd' : 'SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd'[item_def]))
                  (action_usage 'trigger1')
                  (accept_action_usage)
                  (flow_usage 'of')
                  (action_usage 'startEngine'
                    (item_usage in 'ignitionCmd' : 'SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd'[item_def])
                    (item_usage out 'es' : 'SimpleVehicleModel::Definitions::SignalDefinitions::EngineStatus'[item_def]))
                  (flow_usage 'of')
                  (action_usage 'sendStatus')
                  (send_action_usage
                    (reference_usage in reference 'es' : 'SimpleVehicleModel::Definitions::SignalDefinitions::EngineStatus'[item_def]))
                  (action_usage 'trigger2')
                  (accept_action_usage))
                (part_usage composite 'driver' : 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver'[part_def]
                  (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::startVehicle::turnVehicleOn'[action_usage])
                  (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::startVehicle::trigger2'[action_usage])
                  (event_occurrence_usage 'driverReady'))
                (part_usage composite 'vehicle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def]
                  (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::startVehicle::trigger1'[action_usage])
                  (perform_action_usage :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::startVehicle::sendStatus'[action_usage])
                  (event_occurrence_usage 'doorClosed'))
                (succession_def
                  (connector_end 'vehicle.doorClosed')
                  (connector_end 'driver.driverReady'))
                (flow_usage composite 'of')
                (flow_usage composite 'of')))
            (occurrence_usage 'CruiseControl1'
              (part_usage composite 'vehicle_b' :> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b'[part_usage]
                (port_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort'[port_usage]
                  (event_occurrence_usage 'setSpeedReceived'))
                (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor'[part_usage]
                  (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort'[port_usage]
                    (event_occurrence_usage 'sensedSpeedSent')))
                (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware'[part_usage]
                  (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController'[part_usage]
                    (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController'[part_usage]
                      (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort'[port_usage]
                        (event_occurrence_usage 'setSpeedReceived'
                          (feature_value (=))))
                      (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort'[port_usage]
                        (event_occurrence_usage 'sensedSpeedReceived'))
                      (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort'[port_usage]
                        (event_occurrence_usage 'fuelCmdSent')))))
                (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine'[part_usage]
                  (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort'[port_usage]
                    (event_occurrence_usage 'fuelCmdReceived')))
                (flow_usage composite 'sendSensedSpeed' : 'SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed'[item_def]
                  (connector_end 'speedSensor.speedSensorPort.sensedSpeedSent')
                  (connector_end 'vehicleSoftware.vehicleController.cruiseController.speedSensorPort.sensedSpeedReceived'))
                (flow_usage composite 'sendFuelCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd'[item_def]
                  (connector_end 'vehicleSoftware.vehicleController.cruiseController.cruiseControlPort.fuelCmdSent')
                  (connector_end 'engine.fuelCmdPort.fuelCmdReceived'))))
            (occurrence_usage 'CruiseControl2'
              (part_usage composite 'vehicle_b' :> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b'[part_usage]
                (port_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort'[port_usage]
                  (event_occurrence_usage 'setSpeedReceived'))
                (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor'[part_usage]
                  (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort'[port_usage]
                    (not_implemented 'malformed')))
                (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware'[part_usage]
                  (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController'[part_usage]
                    (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController'[part_usage]
                      (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort'[port_usage]
                        (event_occurrence_usage 'setSpeedReceived'
                          (feature_value (=))))
                      (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort'[port_usage]
                        (event_occurrence_usage 'setSpeedReceived'
                          (feature_value (=)))
                        (source_succession
                          (not_implemented 'malformed')))
                      (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort'[port_usage]
                        (not_implemented 'malformed')))))
                (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine'[part_usage]
                  (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort'[port_usage]
                    (not_implemented 'malformed')))
                (flow_usage composite 'sendSensedSpeed' : 'SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed'[item_def])
                (flow_usage composite 'sendFuelCmd' : 'SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd'[item_def]))))
          (package 'Requirements'
            (namespace_import public -> 'RequirementDerivation'[unresolved])
            (namespace_import public -> 'ModelingMetadata'[unresolved])
            (item_usage 'marketSurvey')
            (dependency)
            (requirement_usage 'vehicleSpecification'
              (subject_membership in 'vehicle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def])
              (requirement_usage composite 'vehicleMassRequirement' : 'SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement'[requirement_def]
                (documentation)
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired'[attribute_usage]
                  (feature_value (=)))
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual'[attribute_usage]
                  (feature_value (default =)))
                (attribute_usage composite 'fuelMassActual' :> 'ISQ::mass'[unresolved])
                (attribute_usage composite 'fuelMassMax' :> 'ISQ::mass'[unresolved]
                  (feature_value (=)))
                (assume_constraint_usage composite
                  (result_expr_membership)))
              (allocation_usage composite
                (connector_end 'vehicleMassRequirement')
                (connector_end 'PartsTree::vehicle_b.mass'))
              (requirement_usage composite 'vehicleFuelEconomyRequirements'
                (documentation)
                (attribute_usage composite 'assumedCargoMass' :> 'ISQ::mass'[unresolved])
                (requirement_usage composite 'cityFuelEconomyRequirement' : 'SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement'[requirement_def]
                  (reference_usage reference :>> 'SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::requiredFuelEconomy'[attribute_usage]
                    (feature_value (=)))
                  (assume_constraint_usage composite
                    (result_expr_membership)))
                (requirement_usage composite 'highwayFuelEconomyRequirement' : 'SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement'[requirement_def]
                  (reference_usage reference :>> 'SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::requiredFuelEconomy'[attribute_usage]
                    (feature_value (=)))
                  (assume_constraint_usage composite
                    (result_expr_membership))
                  (metadata_usage :> 'StatusInfo'[unresolved]
                    (feature_def 'status'
                      (feature_value (=)))
                    (feature_def 'originator'
                      (feature_value (=)))
                    (feature_def 'owner'
                      (feature_value (=)))))))
            (requirement_usage 'engineSpecification'
              (subject_membership in 'engine1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine'[part_def])
              (requirement_usage composite 'engineMassRequirement' : 'SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement'[requirement_def]
                (documentation)
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired'[attribute_usage]
                  (feature_value (=)))
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual'[attribute_usage]
                  (feature_value (=))))
              (requirement_usage composite 'torqueGenerationRequirement' : 'SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement'[requirement_def]
                (subject_membership in 'generateTorque'
                  (feature_value (default =))))
              (requirement_usage composite 'drivePowerOutputRequirement' : 'SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement'[requirement_def]
                (port_usage composite 'torqueOutPort'
                  (reference_usage out reference 'torque' : 'SimpleVehicleModel::Definitions::AttributeDefinitions::Torque'[alias_member]))))
            (not_implemented 'malformed')
            (not_implemented 'malformed')))
        (package 'Engine4Cyl_Variant'
          (namespace_import public -> 'ModelingMetadata'[unresolved])
          (part_usage 'engine' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine'[part_def]
            (part_usage composite ordered 'cylinders' : 'SimpleVehicleModel::Definitions::PartDefinitions::Cylinder'[part_def]
              (multiplicity_range [4..8])))
          (part_usage 'engine4Cyl' :> 'SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine'[part_usage]
            (part_usage composite :>> 'SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders'[part_usage]
              (multiplicity_range [4]))
            (part_usage composite 'cylinder1' :> ''[part_usage]
              (multiplicity_range [1]))
            (part_usage composite 'cylinder2' :> ''[part_usage]
              (multiplicity_range [1]))
            (part_usage composite 'cylinder3' :> ''[part_usage]
              (multiplicity_range [1]))
            (part_usage composite 'cylinder4' :> ''[part_usage]
              (multiplicity_range [1])))
          (dependency))
        (package 'WheelHubAssemblies'
          (part_usage 'wheelHubAssy1'
            (part_usage composite 'wheel1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
              (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort'[port_usage] : 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort'[port_def]
                (port_usage composite 'lugNutPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort'[port_usage]
                  (multiplicity_range [5]))))
            (part_usage composite 'hub1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Hub'[part_def]
              (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort'[port_usage] : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort'[port_def]
                (port_usage composite 'shankPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort'[port_usage]
                  (multiplicity_range [5]))))
            (interface_usage composite 'wheelHubInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface'[interface_def]
              (connector_end 'wheel1.lugNutCompositePort')
              (connector_end 'hub1.shankCompositePort')))
          (part_usage 'wheelHubAssy2'
            (part_usage composite 'wheel1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
              (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort'[port_usage] : 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort'[port_def]
                (port_usage composite 'lugNutPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort'[port_usage]
                  (multiplicity_range [5]))))
            (part_usage composite 'hub1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Hub'[part_def]
              (port_usage composite :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort'[port_usage] : 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort'[port_def]
                (port_usage composite 'shankPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort'[port_usage]
                  (multiplicity_range [5]))))
            (interface_usage composite 'wheelHubInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface'[interface_def]
              (connector_end 'lugNutCompositePort' :> ''[port_usage])
              (connector_end 'shankCompositePort' :> ''[port_usage])
              (interface_usage composite 'wheelFastenerInterface1' :> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface'[interface_usage]
                (connector_end 'lugNutPort' :> 'lugNutPort'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::lugNutCompositePort.lugNutPort'[connector_end][implied])
                (connector_end 'shankPort' :> 'shankPort'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::shankCompositePort.shankPort'[connector_end][implied]))))
          (part_usage 'wheelHubAssy3'
            (part_usage composite 'wheel1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel'[part_def]
              (port_usage composite 'lugNutCompositePort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort'[port_usage]
                (port_usage composite 'lugNutPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort'[port_usage]
                  (multiplicity_range [5])
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadDia'[attribute_usage]
                    (feature_value (=)))
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadPitch'[attribute_usage]
                    (feature_value (=))))
                (port_usage composite 'lugNutPort1' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort'[port_usage]
                  (multiplicity_range [1]))
                (port_usage composite 'lugNutPort2' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort'[port_usage]
                  (multiplicity_range [1]))
                (port_usage composite 'lugNutPort3' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort'[port_usage]
                  (multiplicity_range [1]))))
            (part_usage composite 'hub1' : 'SimpleVehicleModel::Definitions::PartDefinitions::Hub'[part_def]
              (port_usage composite 'shankCompositePort' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort'[port_usage]
                (port_usage composite 'shankPort' :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort'[port_usage]
                  (multiplicity_range [5])
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadDia'[attribute_usage]
                    (feature_value (=)))
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadPitch'[attribute_usage]
                    (feature_value (=)))
                  (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::shaftLength'[attribute_usage]
                    (feature_value (=))))
                (port_usage composite 'shankPort1' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort'[port_usage]
                  (multiplicity_range [1]))
                (port_usage composite 'shankPort2' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort'[port_usage]
                  (multiplicity_range [1]))
                (port_usage composite 'shankPort3' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort'[port_usage]
                  (multiplicity_range [1]))))
            (interface_usage composite 'wheelHubInterface' : 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface'[interface_def]
              (connector_end 'lugNutCompositePort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort'[port_usage])
              (connector_end 'shankCompositePort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort'[port_usage])
              (interface_usage composite 'wheelFastenerInterface1' :> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface'[interface_usage]
                (connector_end 'lugNutPort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort1'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::lugNutCompositePort.lugNutPort'[connector_end][implied])
                (connector_end 'shankPort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort1'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::shankCompositePort.shankPort'[connector_end][implied])
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::maxTorque'[attribute_usage]
                  (feature_value (=))))
              (interface_usage composite 'wheelFastenerInterface2' :> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface'[interface_usage]
                (connector_end 'lugNutPort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort2'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::lugNutCompositePort.lugNutPort'[connector_end][implied])
                (connector_end 'shankPort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort2'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::shankCompositePort.shankPort'[connector_end][implied])
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::maxTorque'[attribute_usage]
                  (feature_value (=))))
              (interface_usage composite 'wheelFastenerInterface3' :> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface'[interface_usage]
                (connector_end 'lugNutPort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort3'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::lugNutCompositePort.lugNutPort'[connector_end][implied])
                (connector_end 'shankPort' :> 'SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort3'[port_usage] :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelHubInterface::wheelFastenerInterface::shankCompositePort.shankPort'[connector_end][implied])
                (attribute_usage composite :>> 'SimpleVehicleModel::Definitions::InterfaceDefinitions::WheelFastenerInterface::maxTorque'[attribute_usage]
                  (feature_value (=))))))))
      (package 'VehicleAnalysis'
        (namespace_import public -> 'RiskMetadata'[unresolved])
        (namespace_import public -> 'RiskLevelEnum'[unresolved])
        (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b'[package])
        (package 'FuelEconomyAnalysisModel'
          (membership_import public -> 'SampledFunctions::SampledFunction'[unresolved])
          (attribute_def 'Scenario' :> 'SampledFunction'[unresolved]
            (attribute_usage composite 'wayPoint'
              (multiplicity_range [1..*])
              (attribute_usage composite 'elapseTime' :> 'ISQ::time'[unresolved]
                (multiplicity_range [1]))
              (attribute_usage composite 'position' :> 'ISQ::distance'[unresolved]
                (multiplicity_range [1]))))
          (calculation_def 'FuelConsumption'
            (reference_usage in reference 'bestFuelConsumption' : 'Real'[unresolved])
            (reference_usage in reference 'idlingFuelConsumption' : 'Real'[unresolved])
            (reference_usage in reference 'tpd_avg' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::timePerDistance'[feature_def])
            (attribute_usage composite 'f'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'dpv' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::distancePerVolume'[feature_def]
                (feature_value (=)))))
          (calculation_def 'AverageTravelTimePerDistance'
            (reference_usage in reference 'scenario' : 'SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario'[attribute_def])
            (return_parameter_membership
              (feature_def out 'tpd_avg' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::timePerDistance'[feature_def])))
          (calculation_def 'TraveledDistance'
            (reference_usage in reference 'scenario' : 'SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario'[attribute_def])
            (return_parameter_membership
              (feature_def out 'distance' :> 'length'[unresolved])))
          (calculation_def 'IdlingFuelConsumptionPerTime'
            (reference_usage in reference 'engine' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine'[part_def])
            (attribute_usage composite 'idlingFuelConsumptionPerDisplacement' : 'Real'[unresolved]
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'f_a' : 'Real'[unresolved]
                (feature_value (=)))))
          (attribute_usage 'specificGravityOfGasoline' : 'Real'[unresolved]
            (feature_value (=)))
          (calculation_def 'BestFuelConsumptionPerDistance'
            (reference_usage in reference 'mass' : 'MassValue'[unresolved])
            (reference_usage in reference 'bsfc' : 'Real'[unresolved])
            (reference_usage in reference 'tpd_avg' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::timePerDistance'[feature_def])
            (reference_usage in reference 'distance' :> 'length'[unresolved])
            (attribute_usage composite 'required_power_avg' :> 'ISQ::power'[unresolved])
            (constraint_usage composite
              (result_expr_membership))
            (return_parameter_membership
              (feature_def out 'f_b' : 'Real'[unresolved]
                (feature_value (=)))))
          (calculation_def 'ComputeBSFC'
            (reference_usage in reference 'engine' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine'[part_def])
            (return_parameter_membership
              (feature_def out : 'Real'[unresolved])))
          (analysis_case_usage 'fuelEconomyAnalysis'
            (subject_membership in
              (feature_value (=)))
            (objective_membership composite 'fuelEconomyAnalysisObjective'
              (documentation)
              (not_implemented 'malformed'))
            (attribute_usage in 'scenario' : 'SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario'[attribute_def])
            (attribute_usage composite 'distance'
              (feature_value (=)))
            (attribute_usage composite 'tpd_avg'
              (feature_value (=)))
            (attribute_usage composite 'bsfc'
              (feature_value (=)))
            (attribute_usage composite 'f_a'
              (feature_value (=)))
            (attribute_usage composite 'f_b'
              (feature_value (=)))
            (return_parameter_membership
              (attribute_usage out 'calculatedFuelEconomy' :> 'SimpleVehicleModel::Definitions::AttributeDefinitions::distancePerVolume'[feature_def]
                (feature_value (=))))))
        (package 'ElectricalPowerAnalysis')
        (package 'ReliabilityAnalyis')
        (package 'VehicleTradeOffAnalysis'
          (metadata_usage :> 'Rationale'[unresolved] annotated 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl'[part_usage]
            (feature_def 'explanation'
              (feature_value (=)))
            (feature_def 'text'
              (feature_value (=))))
          (metadata_usage :> 'Risk'[unresolved] annotated 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl'[part_usage]
            (feature_def 'totalRisk'
              (feature_value (=)))
            (feature_def 'technicalRisk'
              (feature_value (=)))
            (feature_def 'scheduleRisk'
              (feature_value (=)))
            (feature_def 'costRisk'
              (feature_value (=))))
          (metadata_usage :> 'Risk'[unresolved] annotated 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::fuelEfficiency'[attribute_usage]
            (feature_def 'technicalRisk'
              (feature_def 'probability'
                (feature_value (=)))
              (feature_def 'impact'
                (feature_value (=)))))
          (namespace_import public -> 'TradeStudies'[unresolved])
          (calculation_def 'EngineEvaluation'
            (reference_usage in reference 'engineMass' :> 'ISQ::mass'[unresolved])
            (reference_usage in reference 'enginePower' :> 'ISQ::power'[unresolved])
            (reference_usage in reference 'engineFuelEfficiency' : 'Real'[unresolved])
            (reference_usage in reference 'engineCost' : 'Real'[unresolved])
            (return_parameter_membership
              (feature_def out 'eval' : 'Real'[unresolved])))
          (calculation_def 'EngineEvaluation_4cyl'
            (reference_usage in reference 'engineMass' :> 'ISQ::mass'[unresolved])
            (reference_usage in reference 'enginePower' :> 'ISQ::power'[unresolved])
            (reference_usage in reference 'engineFuelEfficiency' : 'Real'[unresolved])
            (reference_usage in reference 'engineCost' : 'Real'[unresolved])
            (return_parameter_membership
              (feature_def out 'eval' : 'Real'[unresolved])))
          (calculation_def 'EngineEvaluation_6cyl'
            (reference_usage in reference 'engineMass' :> 'ISQ::mass'[unresolved])
            (reference_usage in reference 'enginePower' :> 'ISQ::power'[unresolved])
            (reference_usage in reference 'engineFuelEfficiency' : 'Real'[unresolved])
            (reference_usage in reference 'engineCost' : 'Real'[unresolved])
            (return_parameter_membership
              (feature_def out 'eval' : 'Real'[unresolved])))
          (analysis_case_usage 'engineTradeOffAnalysis' : 'TradeStudy'[unresolved]
            (subject_membership in 'vehicleAlternatives' :> 'vehicle_b'[unresolved]
              (multiplicity_range [2]))
            (part_usage composite 'vehicle_b_engine4cyl' :> 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicleAlternatives'[subject_membership]
              (part_usage composite 'engine' :>> 'engine'[unresolved]
                (part_usage composite 'cylinders' :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders'[part_usage]
                  (multiplicity_range [4]))
                (attribute_usage composite 'mass' :>> 'mass'[unresolved]
                  (feature_value (=)))
                (attribute_usage composite 'peakHorsePower' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::peakHorsePower'[attribute_usage]
                  (feature_value (=)))
                (attribute_usage composite 'fuelEfficiency' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelEfficiency'[attribute_usage]
                  (feature_value (=)))
                (attribute_usage composite 'cost' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::cost'[attribute_usage]
                  (feature_value (=)))))
            (part_usage composite 'vehicle_b_engine6cyl' :> 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicleAlternatives'[subject_membership]
              (part_usage composite 'engine' :>> 'engine'[unresolved]
                (part_usage composite 'cylinders' :>> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders'[part_usage]
                  (multiplicity_range [6]))
                (attribute_usage composite 'mass' :>> 'mass'[unresolved]
                  (feature_value (=)))
                (attribute_usage composite 'peakHorsePower' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::peakHorsePower'[attribute_usage]
                  (feature_value (=)))
                (attribute_usage composite 'fuelEfficiency' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelEfficiency'[attribute_usage]
                  (feature_value (=)))
                (attribute_usage composite 'cost' :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Engine::cost'[attribute_usage]
                  (feature_value (=)))))
            (objective_membership composite : 'MaximizeObjective'[unresolved])
            (calculation_usage composite :> 'evaluationFunction'[unresolved]
              (part_usage in 'vehicle' :> 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine4cyl'[part_usage])
              (return_parameter_membership
                (attribute_usage out 'eval' : 'Real'[unresolved]
                  (feature_value (=)))))
            (calculation_usage composite :> 'evaluationFunction'[unresolved]
              (part_usage in 'vehicle' :> 'SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis::vehicle_b_engine6cyl'[part_usage])
              (return_parameter_membership
                (attribute_usage out 'eval' : 'Real'[unresolved]
                  (feature_value (=)))))
            (return_parameter_membership
              (part_usage out 'selectedVehicle' :> 'vehicle_b'[unresolved])))))
      (package 'VehicleVerification'
        (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b'[package])
        (namespace_import public -> 'SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions'[package])
        (namespace_import public -> 'SimpleVehicleModel::VehicleVerification::VerificationCases1'[package])
        (namespace_import public -> 'VerificationCases'[unresolved])
        (namespace_import public -> 'SimpleVehicleModel::VehicleVerification::VerificationSystem'[package])
        (package 'VerificationCaseDefinitions'
          (verification_case_def 'MassTest')
          (verification_case_def 'AccelerationTest')
          (verification_case_def 'ReliabilityTest'))
        (package 'VerificationCases1'
          (verification_case_usage 'massTests' : 'SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions::MassTest'[verification_case_def]
            (subject_membership in 'vehicle_uut' :> 'vehicle_b'[unresolved])
            (actor_membership in 'vehicleVerificationSubSystem_1'
              (feature_value (=)))
            (objective_membership composite
              (not_implemented 'malformed'))
            (metadata_usage :> 'VerificationMethod'[unresolved]
              (feature_def 'kind'
                (feature_value (=))))
            (action_usage composite 'weighVehicle'
              (reference_usage out reference 'massMeasured' :> 'ISQ::mass'[unresolved]))
            (source_succession
              (action_usage 'evaluatePassFail'
                (reference_usage in reference 'massMeasured' :> 'ISQ::mass'[unresolved])
                (reference_usage out reference 'verdict'
                  (feature_value (=)))))
            (flow_usage composite
              (connector_end 'weighVehicle.massMeasured')
              (connector_end 'evaluatePassFail.massMeasured'))
            (return_parameter_membership
              (feature_def out :>> 'verdict'[unresolved]
                (feature_value (=))))))
        (package 'VerificationSystem'
          (part_usage 'verificationContext'
            (perform_action_usage :>> 'SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests'[verification_case_usage])
            (part_usage composite 'vehicle_UnitUnderTest' :> 'vehicle_b'[unresolved])
            (part_usage composite 'massVerificationSystem'
              (part_usage composite 'scale'
                (perform_action_usage :>> 'SimpleVehicleModel::VehicleVerification::VerificationCases1::massTests::weighVehicle'[action_usage]))
              (part_usage composite 'operator'
                (perform_action_usage :>> 'massTests::evaluatePassFail'[unresolved]))))))
      (package 'VehicleIndividuals'
        (occurrence_usage individual 'a' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::VehicleRoadContext_1'[occurrence_def]
          (occurrence_usage composite 't0_t2_a'
            (occurrence_usage composite 't0_a'
              (attribute_usage composite 't0' :>> 'SimpleVehicleModel::Definitions::GenericContext::Context::time'[attribute_usage]
                (feature_value (=)))
              (occurrence_usage composite 't0_r' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Road_1'[occurrence_def]
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Road::incline'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Road::friction'[attribute_usage]
                  (feature_value (=))))
              (occurrence_usage composite 't0_v' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Vehicle_1'[occurrence_def]
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration'[attribute_usage]
                  (feature_value (=)))
                (occurrence_usage composite 't0_fa' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxleAssembly_1'[occurrence_def]
                  (occurrence_usage composite 't0_leftFront' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_1'[occurrence_def])
                  (occurrence_usage composite 't0_rightFront' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_2'[occurrence_def]))))
            (occurrence_usage composite 't1_a'
              (attribute_usage composite 't1' :>> 'SimpleVehicleModel::Definitions::GenericContext::Context::time'[attribute_usage]
                (feature_value (=)))
              (occurrence_usage composite 't1_r' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Road_1'[occurrence_def]
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Road::incline'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Road::friction'[attribute_usage]
                  (feature_value (=))))
              (occurrence_usage composite 't1_v' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Vehicle_1'[occurrence_def]
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration'[attribute_usage]
                  (feature_value (=)))
                (occurrence_usage composite 't1_fa' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxleAssembly_1'[occurrence_def]
                  (occurrence_usage composite 't1_leftFront' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_1'[occurrence_def])
                  (occurrence_usage composite 't1_rightFront' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_2'[occurrence_def]))))
            (occurrence_usage composite 't2_a'
              (attribute_usage composite 't2' :>> 'SimpleVehicleModel::Definitions::GenericContext::Context::time'[attribute_usage]
                (feature_value (=)))
              (occurrence_usage composite 't2_r' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Road_1'[occurrence_def]
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Road::incline'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Road::friction'[attribute_usage]
                  (feature_value (=))))
              (occurrence_usage composite 't2_v' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Vehicle_1'[occurrence_def]
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity'[attribute_usage]
                  (feature_value (=)))
                (reference_usage reference :>> 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration'[attribute_usage]
                  (feature_value (=)))
                (occurrence_usage composite 't2_fa' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::FrontAxleAssembly_1'[occurrence_def]
                  (occurrence_usage composite 't2_leftFront' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_1'[occurrence_def])
                  (occurrence_usage composite 't2_rightFront' : 'SimpleVehicleModel::Definitions::IndividualDefinitions::Wheel_2'[occurrence_def])))))))
      (package 'MissionContext'
        (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b'[package])
        (membership_import public -> 'ParametersOfInterestMetadata::moe'[unresolved])
        (namespace_import public -> 'SimpleVehicleModel::MissionContext::TransportPassengerScenario'[package])
        (package 'ContextDefinitions'
          (part_def 'MissionContext' :> 'SimpleVehicleModel::Definitions::GenericContext::Context'[part_def])
          (part_def 'Road')
          (part_def 'Driver'
            (port_usage composite 'handPort' : 'SimpleVehicleModel::Definitions::PortDefinitions::HandPort'[port_def])
            (state_usage composite 'driverStates'
              (state_usage composite 'initial')
              (state_usage composite 'wait')
              (transition_usage)
              (transition_usage)
              (transition_usage)))
          (part_def 'Passenger')
          (requirement_usage 'transportRequirements')
          (use_case_def 'TransportPassenger'
            (objective_membership composite 'TransportObjective'
              (documentation)
              (require_constraint_usage composite 'transportRequirements'))
            (subject_membership in 'vehicle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def])
            (actor_membership in 'environment')
            (actor_membership in 'road')
            (actor_membership in 'driver')
            (actor_membership in 'passenger'
              (multiplicity_range [0..4]))
            (include_use_case_usage 'getInVehicle_a' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::getInVehicle'[use_case_usage]
              (multiplicity_range [1..5]))
            (include_use_case_usage 'getOutOfVehicle_a' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::getOutOfVehicle'[use_case_usage]
              (multiplicity_range [1..5])))
          (use_case_usage 'getInVehicle' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::GetInVehicle'[use_case_def]
            (action_usage composite 'unlockDoor_in'
              (multiplicity_range [0..1]))
            (source_succession
              (action_usage 'openDoor_in'))
            (source_succession
              (action_usage 'enterVehicle'))
            (source_succession
              (action_usage 'closeDoor_in')))
          (use_case_def 'GetInVehicle'
            (subject_membership in 'vehicle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def])
            (actor_membership in 'driver'
              (multiplicity_range [0..1]))
            (actor_membership in 'passenger'
              (multiplicity_range [0..1]))
            (assert_constraint_usage
              (result_expr_membership)))
          (use_case_usage 'getOutOfVehicle' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::GetOutOfVehicle'[use_case_def]
            (action_usage composite 'openDoor_out')
            (source_succession
              (action_usage 'exitVehicle'))
            (source_succession
              (action_usage 'closeDoor_out'))
            (source_succession
              (action_usage 'lockDoor_out')))
          (use_case_def 'GetOutOfVehicle'
            (subject_membership in 'vehicle' : 'SimpleVehicleModel::Definitions::PartDefinitions::Vehicle'[part_def])
            (actor_membership in 'driver'
              (multiplicity_range [0..1]))
            (actor_membership in 'passenger'
              (multiplicity_range [0..1]))
            (assert_constraint_usage
              (result_expr_membership))))
        (package 'TransportPassengerScenario'
          (membership_import public -> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger'[use_case_def])
          (use_case_usage 'transportPassenger' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger'[use_case_def]
            (initial_node)
            (source_succession
              (action_usage 'a'
                (action_usage composite 'driverGetInVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getInVehicle_a'[include_use_case_usage]
                  (multiplicity_range [1]))
                (action_usage composite 'passenger1GetInVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getInVehicle_a'[include_use_case_usage]
                  (multiplicity_range [1]))))
            (source_succession
              (action_usage 'trigger'))
            (accept_action_usage)
            (source_succession
              (action_usage 'b'
                (action_usage composite 'driveVehicleToDestination')
                (action_usage composite 'providePower')))
            (source_succession
              (action_usage 'c'
                (action_usage composite 'driverGetOutOfVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getOutOfVehicle_a'[include_use_case_usage]
                  (multiplicity_range [1]))
                (action_usage composite 'passenger1GetOutOfVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getOutOfVehicle_a'[include_use_case_usage]
                  (multiplicity_range [1]))))
            (source_succession
              (reference_usage reference 'done')))
          (use_case_usage 'transportPassenger_1' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger'[use_case_def]
            (action_usage composite 'driverGetInVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getInVehicle_a'[include_use_case_usage]
              (multiplicity_range [1]))
            (action_usage composite 'passenger1GetInVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getInVehicle_a'[include_use_case_usage]
              (multiplicity_range [1]))
            (action_usage composite 'driverGetOutOfVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getOutOfVehicle_a'[include_use_case_usage]
              (multiplicity_range [1]))
            (action_usage composite 'passenger1GetOutOfVehicle' :> 'SimpleVehicleModel::MissionContext::ContextDefinitions::TransportPassenger::getOutOfVehicle_a'[include_use_case_usage]
              (multiplicity_range [1]))
            (action_usage composite 'driveVehicleToDestination')
            (action_usage composite 'providePower')
            (item_def 'VehicleOnSignal')
            (join_node 'join1')
            (join_node 'join2')
            (join_node 'join3')
            (action_usage composite 'trigger')
            (accept_action_usage)
            (initial_node)
            (source_succession
              (fork_node 'fork1'))
            (source_succession
              (reference_usage reference 'driverGetInVehicle'))
            (source_succession
              (reference_usage reference 'passenger1GetInVehicle'))
            (succession_def
              (connector_end 'driverGetInVehicle')
              (connector_end 'join1'))
            (succession_def
              (connector_end 'passenger1GetInVehicle')
              (connector_end 'join1'))
            (succession_def
              (connector_end 'join1')
              (connector_end 'trigger'))
            (succession_def
              (connector_end 'trigger')
              (connector_end 'fork2'))
            (fork_node 'fork2')
            (source_succession
              (reference_usage reference 'driveVehicleToDestination'))
            (source_succession
              (reference_usage reference 'providePower'))
            (succession_def
              (connector_end 'driveVehicleToDestination')
              (connector_end 'join2'))
            (succession_def
              (connector_end 'providePower')
              (connector_end 'join2'))
            (succession_def
              (connector_end 'join2')
              (connector_end 'fork3'))
            (fork_node 'fork3')
            (source_succession
              (reference_usage reference 'driverGetOutOfVehicle'))
            (source_succession
              (reference_usage reference 'passenger1GetOutOfVehicle'))
            (succession_def
              (connector_end 'driverGetOutOfVehicle')
              (connector_end 'join3'))
            (succession_def
              (connector_end 'passenger1GetOutOfVehicle')
              (connector_end 'join3'))
            (succession_def
              (connector_end 'join3')
              (connector_end 'done'))))
        (part_usage 'missionContext' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext'[part_def]
          (attribute_usage composite 'transportTime' :> 'ISQ::time'[unresolved])
          (perform_action_usage :>> 'SimpleVehicleModel::MissionContext::TransportPassengerScenario::transportPassenger'[use_case_usage])
          (part_usage composite 'road' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::Road'[part_def]
            (feature_value (=)))
          (part_usage composite 'driver' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::Driver'[part_def]
            (feature_value (=))
            (perform_action_usage :>> 'transportPassenger::a::driverGetInVehicle::unlockDoor_in'[unresolved])
            (perform_action_usage :>> 'transportPassenger::a::driverGetInVehicle::openDoor_in'[unresolved])
            (perform_action_usage :>> 'transportPassenger::a::driverGetInVehicle::enterVehicle'[unresolved])
            (perform_action_usage :>> 'transportPassenger::a::driverGetInVehicle::closeDoor_in'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::driverGetOutOfVehicle::openDoor_out'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::driverGetOutOfVehicle::exitVehicle'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::driverGetOutOfVehicle::closeDoor_out'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::driverGetOutOfVehicle::lockDoor_out'[unresolved])
            (perform_action_usage :>> 'transportPassenger::b::driveVehicleToDestination'[unresolved]))
          (part_usage composite 'passenger1' : 'SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger'[part_def]
            (feature_value (=))
            (perform_action_usage :>> 'transportPassenger::a::passenger1GetInVehicle::unlockDoor_in'[unresolved])
            (perform_action_usage :>> 'transportPassenger::a::passenger1GetInVehicle::openDoor_in'[unresolved])
            (perform_action_usage :>> 'transportPassenger::a::passenger1GetInVehicle::enterVehicle'[unresolved])
            (perform_action_usage :>> 'transportPassenger::a::passenger1GetInVehicle::closeDoor_in'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::passenger1GetOutOfVehicle::openDoor_out'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::passenger1GetOutOfVehicle::exitVehicle'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::passenger1GetOutOfVehicle::closeDoor_out'[unresolved])
            (perform_action_usage :>> 'transportPassenger::c::passenger1GetOutOfVehicle::lockDoor_out'[unresolved]))
          (part_usage composite 'vehicle_b_1' :> 'vehicle_b'[unresolved]
            (feature_value (=))
            (attribute_usage composite :>> 'position3dVector'[unresolved]
              (feature_value (=)))
            (perform_action_usage :>> 'transportPassenger::b::providePower'[unresolved])
            (reference_usage reference :>> 'providePower'[unresolved])
            (perform_action_usage :>> 'transportPassenger::trigger'[unresolved]))
          (connection_usage composite
            (connector_end 'driver.handPort')
            (connector_end 'vehicle_b_1.ignitionCmdPort'))
          (connection_usage composite
            (connector_end 'road')
            (connector_end 'vehicle_b_1.vehicleToRoadPort'))))
      (package 'VehicleSuperSetModel'
        (package 'VariationPointDefinitions'
          (part_def variation 'TransmissionChoices' :> 'SimpleVehicleModel::Definitions::PartDefinitions::Transmission'[part_def]
            (variant_usage
              (part_usage composite 'transmissionAutomatic' : 'SimpleVehicleModel::Definitions::PartDefinitions::TransmissionAutomatic'[part_def]))
            (variant_usage
              (part_usage composite 'transmissionManual' : 'SimpleVehicleModel::Definitions::PartDefinitions::TransmissionManual'[part_def]))))
        (package 'VehiclePartsTree'
          (namespace_import public -> 'SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions'[package])
          (part_usage abstract 'vehicleFamily'
            (part_usage variation composite 'engine' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine'[part_def]
              (variant_usage
                (part_usage composite 'engine4Cyl' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine4Cyl'[part_def]))
              (variant_usage
                (part_usage composite 'engine6Cyl' : 'SimpleVehicleModel::Definitions::PartDefinitions::Engine6Cyl'[part_def]
                  (part_usage composite 'cylinder' : 'SimpleVehicleModel::Definitions::PartDefinitions::Cylinder'[part_def]
                    (multiplicity_range [6])
                    (attribute_usage variation composite 'diameter' : 'LengthValue'[unresolved]
                      (variant_usage
                        (attribute_usage composite 'smallDiameter' : 'LengthValue'[unresolved]))
                      (variant_usage
                        (attribute_usage composite 'largeDiagmeter' : 'LengthValue'[unresolved])))))))
            (part_usage composite 'transmissionChoices' : 'SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices'[part_def])
            (part_usage composite 'sunroof' : 'SimpleVehicleModel::Definitions::PartDefinitions::Sunroof'[part_def]
              (multiplicity_range [0..1]))
            (assert_constraint_usage 'selectionConstraint'
              (result_expr_membership))
            (part_usage composite 'driveshaft')
            (part_usage composite 'frontAxleAssembly')
            (part_usage composite 'rearAxleAssembly'))))
      (package 'SafetyandSecurityGroups'
        (namespace_import public -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree'[package])
        (package 'SafetyGroup'
          (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b'[part_usage])
          (element_filter_membership))
        (package 'SecurityGroup'
          (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b'[part_usage])
          (element_filter_membership))
        (package 'SafetyandSecurityGroup'
          (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b'[part_usage])
          (element_filter_membership))
        (package 'MandatorySafetyGroup'
          (membership_import public recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b'[part_usage])
          (element_filter_membership)))
      (package 'Views_Viewpoints'
        (package 'ViewpointDefinitions'
          (viewpoint_def 'BehaviorViewpoint')
          (viewpoint_def 'SafetyViewpoint'
            (framed_concern_membership 'vs' : 'SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::VehicleSafety'[concern_def]))
          (part_def 'SafetyEngineer')
          (concern_def 'VehicleSafety'
            (documentation)
            (subject_membership in)
            (stakeholder_membership in 'se' : 'SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::SafetyEngineer'[part_def])))
        (package 'ViewDefinitions'
          (namespace_import public -> 'Views'[unresolved])
          (view_def 'TreeView'
            (view_rendering_membership -> 'asTreeDiagram'[unresolved]))
          (view_def 'NestedView')
          (view_def 'RelationshipView')
          (view_def 'TableView')
          (view_def 'PartsTreeView' :> 'SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::TreeView'[view_def]
            (element_filter_membership))
          (view_def 'PartsInterconnection' :> 'SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::NestedView'[view_def]))
        (package 'VehicleViews'
          (namespace_import public -> 'SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions'[package])
          (namespace_import public -> 'SimpleVehicleModel::Views_Viewpoints::ViewDefinitions'[package])
          (namespace_import public -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b'[package])
          (view_usage 'vehiclePartsTree_Safety' : 'SimpleVehicleModel::Views_Viewpoints::ViewDefinitions::PartsTreeView'[view_def]
            (satisfy_requirement_usage 'sv' : 'SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::SafetyViewpoint'[viewpoint_def])
            (namespace_expose all recursive -> 'SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree'[package])
            (element_filter_membership)))))))
~~~
