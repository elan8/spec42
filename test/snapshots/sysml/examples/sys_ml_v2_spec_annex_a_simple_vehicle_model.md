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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 18) (end 3 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 16 22) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 20 34) (end 20 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 21 35) (end 21 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 22 37) (end 22 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 23 36) (end 23 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 24 36) (end 24 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 25 40) (end 25 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 26 43) (end 26 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 27 32) (end 27 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 43) (end 28 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 47) (end 29 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 34 16) (end 34 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 35 16) (end 35 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 36 16) (end 36 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 37 16) (end 37 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 38 16) (end 38 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 39 16) (end 39 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 40 16) (end 107 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 110 34) (end 110 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 111 42) (end 111 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 41) (end 112 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 31) (end 113 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 114 42) (end 114 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 121 16) (end 121 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 122 16) (end 128 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 36) (end 135 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 137 16) (end 137 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 142 32) (end 142 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 145 41) (end 145 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 35) (end 153 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 163 16) (end 178 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 184 16) (end 184 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 190 34) (end 190 43))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 191 16) (end 194 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 195 39) (end 195 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 196 16) (end 196 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 207 34) (end 207 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 208 35) (end 208 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 225 16) (end 225 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 230 16) (end 230 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 233 16) (end 233 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 236 16) (end 236 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 239 16) (end 239 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 274 16) (end 274 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 277 16) (end 277 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 278 16) (end 278 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 283 40) (end 283 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 287 36) (end 287 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 290 33) (end 290 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 309 36) (end 309 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 312 12) (end 316 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 317 12) (end 321 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 323 12) (end 328 13))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 327 16) (end 328 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 329 12) (end 334 13))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 332 16) (end 334 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 337 12) (end 340 13))
      )
      (diagnostic
        (severity error)
        (code "recovered_definition_body_element")
        (source "parser")
        (range (start 338 16) (end 339 16))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 338 16) (end 339 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 345 16) (end 345 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 349 16) (end 349 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 352 16) (end 352 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 353 16) (end 353 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 356 16) (end 356 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 357 16) (end 357 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 360 16) (end 360 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 361 16) (end 361 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 366 16) (end 366 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 370 12) (end 370 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 371 12) (end 371 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 372 12) (end 372 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 377 40) (end 377 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 378 38) (end 378 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 379 16) (end 379 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 383 46) (end 383 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 384 44) (end 384 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 385 16) (end 385 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 389 16) (end 389 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 397 47) (end 397 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 398 49) (end 398 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 399 16) (end 399 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 403 26) (end 403 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 404 26) (end 404 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 405 26) (end 405 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 406 26) (end 406 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 408 26) (end 408 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 409 26) (end 409 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 410 26) (end 410 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 411 29) (end 411 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 414 38) (end 414 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 423 12) (end 423 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 424 12) (end 424 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 425 12) (end 425 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 426 12) (end 426 62))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 429 12) (end 433 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 435 12) (end 435 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 436 12) (end 436 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 437 12) (end 437 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 438 12) (end 438 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 439 12) (end 439 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 440 12) (end 440 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 441 12) (end 441 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 442 12) (end 442 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 445 26) (end 445 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 446 12) (end 448 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 449 12) (end 449 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 452 26) (end 452 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 455 12) (end 455 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 458 12) (end 460 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 462 23) (end 462 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 464 12) (end 466 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 468 23) (end 468 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 470 12) (end 472 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 477 31) (end 477 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 478 37) (end 478 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 478 80) (end 478 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 479 38) (end 479 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 480 42) (end 480 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 487 12) (end 487 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 500 22) (end 500 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 503 8) (end 511 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 517 45) (end 517 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 518 48) (end 518 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 519 40) (end 519 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 520 57) (end 520 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 522 44) (end 522 48))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 523 24) (end 526 20))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 523 24) (end 526 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 528 42) (end 528 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 533 42) (end 533 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 534 55) (end 534 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 537 48) (end 537 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 549 26) (end 549 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 550 26) (end 550 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 551 26) (end 551 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 555 20) (end 555 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 556 48) (end 556 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 557 40) (end 557 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 559 48) (end 559 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 560 60) (end 560 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 564 53) (end 564 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 572 20) (end 572 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 575 47) (end 575 50))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 576 24) (end 577 24))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 577 24) (end 578 24))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 578 24) (end 579 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 582 44) (end 582 48))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 583 24) (end 586 24))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 583 24) (end 586 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 586 44) (end 586 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 589 42) (end 589 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 596 42) (end 596 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 597 55) (end 597 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 599 24) (end 599 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 601 48) (end 601 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 603 57) (end 603 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 608 48) (end 608 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 610 57) (end 610 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 622 60) (end 622 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 628 60) (end 628 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 634 24) (end 634 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 635 24) (end 635 102))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 636 24) (end 636 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 638 24) (end 639 119))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 640 24) (end 641 120))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 651 24) (end 658 25))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 653 32) (end 654 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 656 32) (end 657 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 661 42) (end 661 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 663 24) (end 663 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 666 42) (end 666 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 669 24) (end 669 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 673 28) (end 673 86))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 682 42) (end 682 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 684 37) (end 684 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 685 43) (end 685 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 688 36) (end 688 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 689 42) (end 689 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 691 43) (end 691 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 695 20) (end 695 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 697 20) (end 698 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 700 20) (end 701 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 703 20) (end 703 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 705 20) (end 705 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 706 20) (end 706 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 707 20) (end 707 102))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 708 20) (end 708 114))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 709 20) (end 709 126))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 710 20) (end 710 142))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 711 20) (end 711 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 712 20) (end 712 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 713 20) (end 713 104))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 714 20) (end 714 104))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 716 20) (end 721 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 726 54) (end 726 60))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 727 20) (end 728 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 736 20) (end 736 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 738 20) (end 738 95))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 739 20) (end 739 94))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 753 24) (end 768 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 770 28) (end 770 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 771 28) (end 771 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 772 28) (end 772 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 775 28) (end 775 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 776 28) (end 776 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 777 28) (end 777 56))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 779 24) (end 780 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 780 24) (end 780 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 781 24) (end 781 94))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 784 27) (end 820 17))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 787 28) (end 788 24))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 791 32) (end 792 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 799 40) (end 800 36))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 802 40) (end 803 36))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 805 40) (end 806 36))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 812 32) (end 813 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 821 27) (end 856 17))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 824 28) (end 825 24))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 828 32) (end 829 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 836 40) (end 837 36))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 839 40) (end 840 40))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 840 40) (end 841 36))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 843 40) (end 844 36))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 850 32) (end 851 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 859 30) (end 859 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 860 30) (end 860 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 862 16) (end 862 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 865 20) (end 865 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 869 44) (end 869 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 870 44) (end 870 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 871 50) (end 871 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 872 47) (end 872 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 873 24) (end 873 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 876 20) (end 876 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 880 52) (end 880 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 882 38) (end 882 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 883 28) (end 883 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 886 38) (end 886 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 887 28) (end 887 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 891 28) (end 895 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 900 20) (end 900 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 903 44) (end 903 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 904 44) (end 904 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 907 24) (end 907 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 911 24) (end 913 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 917 16) (end 920 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 925 26) (end 925 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 929 29) (end 929 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 930 31) (end 930 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 931 39) (end 931 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 932 39) (end 932 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 933 39) (end 933 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 934 39) (end 934 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 936 12) (end 936 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 936 24) (end 936 102))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 942 28) (end 942 47))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 943 24) (end 944 20))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 943 24) (end 944 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 947 29) (end 947 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 951 16) (end 952 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 957 28) (end 957 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 962 29) (end 962 47))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 966 16) (end 967 20))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 967 20) (end 971 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 975 49) (end 975 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 976 48) (end 976 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 977 42) (end 977 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 978 42) (end 978 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 980 48) (end 980 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 981 48) (end 981 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 982 48) (end 982 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 986 48) (end 986 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 987 47) (end 987 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 988 42) (end 988 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 989 42) (end 989 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 990 42) (end 990 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 992 47) (end 992 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 993 47) (end 993 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 994 47) (end 994 56))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 997 16) (end 998 20))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 998 20) (end 1012 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1016 22) (end 1016 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1017 22) (end 1017 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1019 22) (end 1019 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1021 26) (end 1021 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1047 38) (end 1047 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1049 47) (end 1049 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1050 45) (end 1050 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1054 12) (end 1060 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1062 12) (end 1065 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1066 12) (end 1069 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1070 12) (end 1074 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1076 49) (end 1076 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1077 12) (end 1085 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1087 12) (end 1090 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1092 12) (end 1110 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 1120 12) (end 1141 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1141 26) (end 1141 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1143 12) (end 1149 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1150 12) (end 1156 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1157 12) (end 1163 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1164 12) (end 1198 13))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 1172 24) (end 1173 24))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 1172 24) (end 1173 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1202 22) (end 1202 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1206 22) (end 1206 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1209 12) (end 1209 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1210 12) (end 1210 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1211 12) (end 1211 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1214 12) (end 1235 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1239 16) (end 1239 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1240 46) (end 1240 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1243 24) (end 1243 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1246 24) (end 1246 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1253 19) (end 1307 9))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "parser")
        (range (start 1259 24) (end 1260 20))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 1259 24) (end 1260 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1311 22) (end 1311 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1312 22) (end 1312 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 1320 16) (end 1334 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1339 12) (end 1351 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1353 12) (end 1358 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1359 12) (end 1364 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1366 12) (end 1371 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1372 12) (end 1377 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1380 26) (end 1380 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1383 12) (end 1399 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1403 12) (end 1442 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1446 12) (end 1446 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1447 12) (end 1447 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1451 16) (end 1451 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1452 16) (end 1452 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1453 16) (end 1453 77))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1454 16) (end 1454 77))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1455 16) (end 1455 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1456 16) (end 1456 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1457 16) (end 1457 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1458 16) (end 1458 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1459 16) (end 1459 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1462 16) (end 1462 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1463 16) (end 1463 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1464 16) (end 1464 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1465 16) (end 1465 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1466 16) (end 1466 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1467 16) (end 1467 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1468 16) (end 1468 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1469 16) (end 1469 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1471 30) (end 1471 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1472 30) (end 1472 46))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 1473 16) (end 1474 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1474 16) (end 1474 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1476 12) (end 1476 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1477 12) (end 1477 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 1484 16) (end 1484 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 1485 16) (end 1485 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1493 20) (end 1493 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1494 20) (end 1501 21))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 1497 32) (end 1498 32))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 1498 32) (end 1499 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 1508 16) (end 1511 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1522 26) (end 1522 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1523 12) (end 1523 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1527 26) (end 1527 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1528 12) (end 1528 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1532 26) (end 1532 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1533 12) (end 1533 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 1537 26) (end 1537 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1538 12) (end 1538 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1543 12) (end 1543 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1544 12) (end 1546 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1548 12) (end 1552 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1556 26) (end 1556 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1557 12) (end 1559 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1560 12) (end 1560 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1561 12) (end 1561 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1562 12) (end 1562 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1563 12) (end 1565 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1566 12) (end 1566 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1572 12) (end 1576 13))
      )
      (diagnostic
        (severity error)
        (code "recovered_view_body_element")
        (source "parser")
        (range (start 1573 16) (end 1574 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:90be26144f137fa77c52045c140f97a4dd6322eb6ab6064954b9b91f6bb9d5b5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "PartDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "PortDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ItemDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "SignalDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "InterfaceDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AllocationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ActionDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "StateDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "RequirementDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AttributeDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "IndividualDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "MetadataDefinitions") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "KeyWord_MetadataDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PwrCmd"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AllocationDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "MeasurementReferences::DerivedUnit") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "SIPrefixes::kilo") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors::black"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors::grey"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors::red"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ISQ::LengthValue"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind::diesel"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind::gas"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff::off"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff::on"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DiameterChoices"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::accelarationCF"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianAcceleration3dCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::spatialCF"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSpatial3dCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mRefs"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::time"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::velocityCF"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVelocity3dCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel::fuelMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PwrCmd"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd::throttleLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed::speed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::MetadataDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AnalysisTooling") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Colors"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Software"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseControlPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SetSpeedPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedSensorPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine4Cyl"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine6Cyl"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::cost"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::displacement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::volume"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControlPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::flyWheelPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IgnitionCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::peakHorsePower"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle::steeringAngle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::angularMeasure"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelKind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::friction"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::incline"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedSensorPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GearPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Sunroof"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Thermostat"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::gearRatio"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionAutomatic"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionChoices"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TransmissionManual"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::Tmax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::temperature"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::acceleration"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::brakePedalDepressed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::electricalPower"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IgnitionCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Time::DateTime"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::length"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PwrCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StatusPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Software"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControlPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Software"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::WaterHose"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugNutCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxleToWheelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PwrCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DriverCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugNutPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadDia"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort::threadPitch"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShankPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::shaftLength"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadDia"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort::threadPitch"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToAxlePort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::actualFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::requiredFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityActual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityRequired"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::Cmd"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::EngineStatus"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DriverCmd"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IgnitionOnOff"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::OffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::OverTemp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::ReturnToNormal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::SetSpeed"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::StartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "VehicleConfigurations::VehicleConfiguration_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "ParametersOfInterestMetadata::moe") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "TransportPassengerScenario") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HandPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "GenericContext::Context"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Road"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::transportRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "ContextDefinitions::TransportPassenger") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContextDefinitions::MissionContext"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContextDefinitions::Driver"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContextDefinitions::Passenger"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContextDefinitions::Road"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle_b"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "position3dVector"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VehicleConfigurations::VehicleConfiguration_b::PartsTree") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::MandatorySafetyGroup"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyGroup"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SafetyandSecurityGroup"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::SafetyandSecurityGroups::SecurityGroup"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "RiskMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "RiskLevelEnum") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "VehicleConfigurations::VehicleConfiguration_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::ElectricalPowerAnalysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "SampledFunctions::SampledFunction") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SampledFunction"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint::elapseTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::time"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint::position"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::distance"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::specificGravityOfGasoline"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::ReliabilityAnalyis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::VehicleTradeOffAnalysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "TradeStudies") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ModelingMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cylinders"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "cylinders"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "cylinders"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "cylinders"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "cylinders"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::ActionTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::cargoMass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::dryMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::dryMass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTank"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::partMasses"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::driveTrainEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "diameter"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "ShapeItems::Box") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "ParametersOfInterestMetadata::mop") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ModelingMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ApplyParkingBrake"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformSelfTest"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ProvidePower"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AmplifyTorque"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistributeTorque"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd")) (redefinition (reference "pwrCmd"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransferTorque"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SenseTemperature"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver::p1"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver::p2"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cargoMass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::avgFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BodyAssy"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Body"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "color"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driveshaft"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_b"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_c"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::dryMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dryMass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::alternator"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::alternator::generateElectricity"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FrontAxle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmdPort")) (redefinition (reference "pwrCmdPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTank"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelMassMax"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::partMasses"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Differential"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiffPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiffPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::driveTrainEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HalfAxle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxlePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::shankCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HalfAxle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxlePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::shankCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "diameter"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::lugNutCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lugNutCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "diameter"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::lugNutCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lugNutCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SetSpeedPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedSensor"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StarterMotor"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_a"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleSoftware"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicleToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "RequirementDerivation") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ModelingMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePowerOutputRequirement"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRequirement"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massRequired"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massActual"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerationRequirement"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::marketSurvey"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::assumedCargoMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRequirement"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massRequired"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massActual"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassActual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassMax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hub"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShankCompositePort")) (redefinition (reference "shankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugNutCompositePort")) (redefinition (reference "lugNutCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hub"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShankCompositePort")) (redefinition (reference "shankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugNutCompositePort")) (redefinition (reference "lugNutCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hub"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shankCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shankPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "shankPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "shankPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort3"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "shankPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "threadDia"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "threadPitch"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftLength"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lugNutCompositePort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lugNutPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "lugNutPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "lugNutPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort3"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "lugNutPort"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "threadDia"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "threadPitch"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleIndividuals"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BrakingSubsystem"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricalGenerator"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator::generateElectricity"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SteeringSubsystem"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerator"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator::generateTorque"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalToPhysicalAllocation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "VehicleConfigurations::VehicleConfiguration_b::PartsTree") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VehicleLogicalConfiguration::PartsTree") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VariationPointDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::driveshaft"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sunroof"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransmissionChoices"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "VehicleConfigurations::VehicleConfiguration_b") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VerificationCaseDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VerificationCases1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VerificationCases") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VerificationSystem") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::operator"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::massVerificationSystem::scale"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::vehicle_UnitUnderTest"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle_b"))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::VehicleViews"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ViewpointDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ViewDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VehicleConfigurations::VehicleConfiguration_b") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Views") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions::SafetyEngineer"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PartDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PortDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ItemDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SignalDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "InterfaceDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AllocationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AllocationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ActionDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 7))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StateDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 9))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AttributeDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 10))))) (kind namespaceImport) (ordinal 0))
      (authored-target "IndividualDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 12))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KeyWord_MetadataDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "MetadataDefinitions")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "SIPrefixes::kilo")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices"))) (kind specialization) (ordinal 0))
      (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiameterChoices")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::accelarationCF"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianAcceleration3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::spatialCF"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::time"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::velocityCF"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVelocity3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel::fuelMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (kind specialization) (ordinal 0))
      (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd::throttleLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed::speed"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnalysisTooling")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (kind featureTyping) (ordinal 0))
      (authored-target "Colors")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (kind specialization) (ordinal 0))
      (authored-target "Software")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SetSpeedPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedSensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::cost"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::displacement"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::volume")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "IgnitionCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::peakHorsePower"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (kind specialization) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle::steeringAngle"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::angularMeasure")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::friction"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::incline"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedSensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "GearPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::gearRatio"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::Tmax"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::temperature")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::acceleration")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::brakePedalDepressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::electricalPower"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "IgnitionCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::length")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PwrCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "StatusPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (kind specialization) (ordinal 0))
      (authored-target "Software")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (kind specialization) (ordinal 0))
      (authored-target "Software")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugNutCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (kind specialization) (ordinal 0))
      (authored-target "ControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (kind specialization) (ordinal 0))
      (authored-target "PwrCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (kind specialization) (ordinal 0))
      (authored-target "DriverCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugNutPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShankPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityActual"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityRequired"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (kind specialization) (ordinal 0))
      (authored-target "DriverCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (kind featureTyping) (ordinal 0))
      (authored-target "IgnitionOnOff")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::SetSpeed"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TransportPassengerScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ParametersOfInterestMetadata::moe")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "HandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (kind specialization) (ordinal 0))
      (authored-target "GenericContext::Context")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ContextDefinitions::TransportPassenger")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContextDefinitions::MissionContext")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContextDefinitions::Driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContextDefinitions::Passenger")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContextDefinitions::Road")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Road")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "position3dVector")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleConfigurations::VehicleConfiguration_b::PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SampledFunctions::SampledFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))) (kind specialization) (ordinal 0))
      (authored-target "SampledFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint::elapseTime"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::time")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint::position"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::distance")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::specificGravityOfGasoline"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TradeStudies")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cylinders")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder1"))) (kind subsetting) (ordinal 0))
      (authored-target "cylinders")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder2"))) (kind subsetting) (ordinal 0))
      (authored-target "cylinders")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder3"))) (kind subsetting) (ordinal 0))
      (authored-target "cylinders")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder4"))) (kind subsetting) (ordinal 0))
      (authored-target "cylinders")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::cargoMass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::dryMass"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::dryMass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::partMasses"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "diameter")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ShapeItems::Box")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ParametersOfInterestMetadata::mop")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (kind featureTyping) (ordinal 0))
      (authored-target "ApplyParkingBrake")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (kind featureTyping) (ordinal 0))
      (authored-target "ProvidePower")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmplifyTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistributeTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (kind redefinition) (ordinal 0))
      (authored-target "pwrCmd")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransferTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "SenseTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cargoMass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::avgFuelEconomy"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "BodyAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (kind featureTyping) (ordinal 0))
      (authored-target "Body")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "color")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_c")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::dryMass"))) (kind redefinition) (ordinal 0))
      (authored-target "dryMass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "FrontAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (kind redefinition) (ordinal 0))
      (authored-target "pwrCmdPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelMassMax")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0))
      (authored-target "Differential")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::shankCompositePort"))) (kind redefinition) (ordinal 0))
      (authored-target "shankCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::shankCompositePort"))) (kind redefinition) (ordinal 0))
      (authored-target "shankCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "diameter")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::lugNutCompositePort"))) (kind redefinition) (ordinal 0))
      (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "diameter")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::lugNutCompositePort"))) (kind redefinition) (ordinal 0))
      (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SetSpeedPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedSensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (kind featureTyping) (ordinal 0))
      (authored-target "StarterMotor")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleSoftware")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicleToRoadPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePowerOutputRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massRequired")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "massActual")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerationRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::assumedCargoMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredFuelEconomy")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredFuelEconomy")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massRequired")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "massActual")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassActual"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassMax"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shankCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "LugNutCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shankCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "LugNutCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort"))) (kind redefinition) (ordinal 0))
      (authored-target "shankCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))) (kind redefinition) (ordinal 0))
      (authored-target "shankPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort1"))) (kind subsetting) (ordinal 0))
      (authored-target "shankPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort2"))) (kind subsetting) (ordinal 0))
      (authored-target "shankPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort3"))) (kind subsetting) (ordinal 0))
      (authored-target "shankPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "threadDia")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "threadPitch")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftLength")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort"))) (kind redefinition) (ordinal 0))
      (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))) (kind redefinition) (ordinal 0))
      (authored-target "lugNutPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort1"))) (kind subsetting) (ordinal 0))
      (authored-target "lugNutPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort2"))) (kind subsetting) (ordinal 0))
      (authored-target "lugNutPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort3"))) (kind subsetting) (ordinal 0))
      (authored-target "lugNutPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "threadDia")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "threadPitch")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "BrakingSubsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricalGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "SteeringSubsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleLogicalConfiguration::PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleConfigurations::VehicleConfiguration_b::PartsTree")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (kind specialization) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VariationPointDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sunroof")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Sunroof")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransmissionChoices")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationCaseDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationCases1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationCases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::vehicle_UnitUnderTest"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ViewpointDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Views")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Road"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Sunroof"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 2 18) (end 2 32)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 3 18) (end 3 24)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 5 22) (end 5 40)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "PartDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 6 22) (end 6 40)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "PortDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 7 22) (end 7 40)) (probe (position 7 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ItemDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 8 22) (end 8 42)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "SignalDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 9 22) (end 9 45)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "InterfaceDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::InterfaceDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 10 22) (end 10 46)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0) (authored-target "AllocationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AllocationDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 11 22) (end 11 42)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0) (authored-target "ActionDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 12 22) (end 12 41)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 7))))) (kind namespaceImport) (ordinal 0) (authored-target "StateDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::StateDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 13 22) (end 13 47)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 14 22) (end 14 45)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 9))))) (kind namespaceImport) (ordinal 0) (authored-target "AttributeDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 15 22) (end 15 46)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 10))))) (kind namespaceImport) (ordinal 0) (authored-target "IndividualDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::IndividualDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 17 22) (end 17 52)) (probe (position 17 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 12))))) (kind namespaceImport) (ordinal 0) (authored-target "KeyWord_MetadataDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::KeyWord_MetadataDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 16 22) (end 16 45)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "MetadataDefinitions")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 348 32) (end 348 39)) (probe (position 348 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 344 31) (end 344 37)) (probe (position 344 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower::pwrCmd"))) (kind featureTyping) (ordinal 0) (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 403 26) (end 403 41)) (probe (position 403 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 404 26) (end 404 39)) (probe (position 404 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 408 26) (end 408 47)) (probe (position 408 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 409 26) (end 409 31)) (probe (position 409 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 410 26) (end 410 45)) (probe (position 410 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 405 26) (end 405 60)) (probe (position 405 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 406 26) (end 406 42)) (probe (position 406 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SIPrefixes::kilo")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 414 38) (end 414 54)) (probe (position 414 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices"))) (kind specialization) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 411 29) (end 411 45)) (probe (position 411 29))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 419 40) (end 419 55)) (probe (position 419 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::cylinderDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "DiameterChoices")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::DiameterChoices")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 480 42) (end 480 80)) (probe (position 480 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::accelarationCF"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianAcceleration3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 478 37) (end 478 70)) (probe (position 478 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::spatialCF"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 478 80) (end 478 85)) (probe (position 478 80))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 477 31) (end 477 40)) (probe (position 477 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::time"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 479 38) (end 479 72)) (probe (position 479 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context::velocityCF"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVelocity3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 287 36) (end 287 45)) (probe (position 287 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::Fuel::fuelMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 285 30) (end 285 36)) (probe (position 285 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd"))) (kind specialization) (ordinal 0) (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 283 40) (end 283 44)) (probe (position 283 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::PwrCmd::throttleLevel"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 290 33) (end 290 43)) (probe (position 290 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::SensedSpeed::speed"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 452 26) (end 452 55)) (probe (position 452 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 445 26) (end 445 44)) (probe (position 445 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AnalysisTooling")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 142 32) (end 142 41)) (probe (position 142 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 202 32) (end 202 38)) (probe (position 202 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body::color"))) (kind featureTyping) (ordinal 0) (authored-target "Colors")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::Colors")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 180 39) (end 180 47)) (probe (position 180 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController"))) (kind specialization) (ordinal 0) (authored-target "Software")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 183 39) (end 183 56)) (probe (position 183 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::cruiseControlPort"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 181 35) (end 181 47)) (probe (position 181 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::setSpeedPort"))) (kind featureTyping) (ordinal 0) (authored-target "SetSpeedPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 182 38) (end 182 53)) (probe (position 182 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController::speedSensorPort"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedSensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 113 31) (end 113 35)) (probe (position 113 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::cost"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 114 42) (end 114 53)) (probe (position 114 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::displacement"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::volume")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 118 34) (end 118 46)) (probe (position 118 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 115 41) (end 115 52)) (probe (position 115 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::engineControlPort"))) (kind featureTyping) (ordinal 0) (authored-target "ControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 117 33) (end 117 44)) (probe (position 117 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 112 41) (end 112 45)) (probe (position 112 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 116 35) (end 116 43)) (probe (position 116 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 119 37) (end 119 52)) (probe (position 119 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::ignitionCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "IgnitionCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 110 34) (end 110 43)) (probe (position 110 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 111 42) (end 111 52)) (probe (position 111 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine::peakHorsePower"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 144 32) (end 144 36)) (probe (position 144 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle"))) (kind specialization) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 145 41) (end 145 60)) (probe (position 145 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle::steeringAngle"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::angularMeasure")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 198 33) (end 198 41)) (probe (position 198 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 194 35) (end 194 43)) (probe (position 194 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelKind"))) (kind featureTyping) (ordinal 0) (authored-target "FuelKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::FuelKind")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 195 39) (end 195 48)) (probe (position 195 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelMassMax"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 197 33) (end 197 41)) (probe (position 197 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 190 34) (end 190 43)) (probe (position 190 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 148 40) (end 148 58)) (probe (position 148 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle::shankCompositePort"))) (kind featureTyping) (ordinal 0) (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 157 40) (end 157 58)) (probe (position 157 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub::shankCompositePort"))) (kind featureTyping) (ordinal 0) (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 208 35) (end 208 39)) (probe (position 208 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::friction"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 207 34) (end 207 38)) (probe (position 207 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Road::incline"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 187 37) (end 187 52)) (probe (position 187 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor::speedSensorPort"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedSensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SpeedSensorPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 131 30) (end 131 38)) (probe (position 131 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor::gearPort"))) (kind featureTyping) (ordinal 0) (authored-target "GearPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::GearPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 136 33) (end 136 45)) (probe (position 136 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DrivePwrPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 135 36) (end 135 40)) (probe (position 135 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission::gearRatio"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 27 32) (end 27 48)) (probe (position 27 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::Tmax"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::temperature")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 25 40) (end 25 57)) (probe (position 25 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::acceleration"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::acceleration")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 29 47) (end 29 54)) (probe (position 29 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::brakePedalDepressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 22 37) (end 22 46)) (probe (position 22 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::cargoMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 21 35) (end 21 44)) (probe (position 21 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::dryMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 26 43) (end 26 53)) (probe (position 26 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::electricalPower"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 30 37) (end 30 52)) (probe (position 30 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::ignitionCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "IgnitionCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::IgnitionCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 28 43) (end 28 57)) (probe (position 28 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 20 34) (end 20 43)) (probe (position 20 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 23 36) (end 23 47)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::position"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::length")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 31 32) (end 31 42)) (probe (position 31 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::pwrCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "PwrCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 33 32) (end 33 42)) (probe (position 33 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::statusPort"))) (kind featureTyping) (ordinal 0) (authored-target "StatusPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::StatusPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 32 39) (end 32 56)) (probe (position 32 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::VehicleToRoadPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 24 36) (end 24 46)) (probe (position 24 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle::velocity"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 161 40) (end 161 48)) (probe (position 161 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController"))) (kind specialization) (ordinal 0) (authored-target "Software")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 162 33) (end 162 44)) (probe (position 162 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController::controlPort"))) (kind featureTyping) (ordinal 0) (authored-target "ControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 160 38) (end 160 46)) (probe (position 160 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware"))) (kind specialization) (ordinal 0) (authored-target "Software")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Software")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 153 35) (end 153 46)) (probe (position 153 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::diameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 154 41) (end 154 60)) (probe (position 154 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel::lugNutCompositePort"))) (kind featureTyping) (ordinal 0) (authored-target "LugNutCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 269 40) (end 269 51)) (probe (position 269 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::CruiseControlPort"))) (kind specialization) (ordinal 0) (authored-target "ControlPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ControlPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 232 34) (end 232 44)) (probe (position 232 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort"))) (kind specialization) (ordinal 0) (authored-target "PwrCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::PwrCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 276 33) (end 276 46)) (probe (position 276 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort"))) (kind specialization) (ordinal 0) (authored-target "DriverCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DriverCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 252 32) (end 252 42)) (probe (position 252 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort::lugNutPort"))) (kind featureTyping) (ordinal 0) (authored-target "LugNutPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 255 31) (end 255 40)) (probe (position 255 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort::shankPort"))) (kind featureTyping) (ordinal 0) (authored-target "ShankPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 397 47) (end 397 64)) (probe (position 397 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 398 49) (end 398 66)) (probe (position 398 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 378 38) (end 378 47)) (probe (position 378 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massActual"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 377 40) (end 377 49)) (probe (position 377 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement::massRequired"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 384 44) (end 384 48)) (probe (position 384 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityActual"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 383 46) (end 383 50)) (probe (position 383 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::ReliabilityRequirement::reliabilityRequired"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 297 34) (end 297 43)) (probe (position 297 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd"))) (kind specialization) (ordinal 0) (authored-target "DriverCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::DriverCmd")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 298 40) (end 298 53)) (probe (position 298 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::IgnitionCmd::ignitionOnOff"))) (kind featureTyping) (ordinal 0) (authored-target "IgnitionOnOff")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::AttributeDefinitions::IgnitionOnOff")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 309 36) (end 309 40)) (probe (position 309 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::SignalDefinitions::SetSpeed"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1313 22) (end 1313 51)) (probe (position 1313 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "TransportPassengerScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::TransportPassengerScenario")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1311 22) (end 1311 71)) (probe (position 1311 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1312 22) (end 1312 55)) (probe (position 1312 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ParametersOfInterestMetadata::moe")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1318 30) (end 1318 38)) (probe (position 1318 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver::handPort"))) (kind featureTyping) (ordinal 0) (authored-target "HandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::HandPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1315 37) (end 1315 60)) (probe (position 1315 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext"))) (kind specialization) (ordinal 0) (authored-target "GenericContext::Context")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::GenericContext::Context")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1380 26) (end 1380 64)) (probe (position 1380 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ContextDefinitions::TransportPassenger")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1445 28) (end 1445 62)) (probe (position 1445 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext"))) (kind featureTyping) (ordinal 0) (authored-target "ContextDefinitions::MissionContext")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::MissionContext")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1450 24) (end 1450 50)) (probe (position 1450 24))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::driver"))) (kind featureTyping) (ordinal 0) (authored-target "ContextDefinitions::Driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Driver")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1461 28) (end 1461 57)) (probe (position 1461 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::passenger1"))) (kind featureTyping) (ordinal 0) (authored-target "ContextDefinitions::Passenger")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Passenger")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1449 22) (end 1449 46)) (probe (position 1449 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::road"))) (kind featureTyping) (ordinal 0) (authored-target "ContextDefinitions::Road")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::ContextDefinitions::Road")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1471 30) (end 1471 39)) (probe (position 1471 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::MissionContext::missionContext::vehicle_b_1"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1472 30) (end 1472 46)) (probe (position 1472 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "position3dVector")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1519 22) (end 1519 81)) (probe (position 1519 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleConfigurations::VehicleConfiguration_b::PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1537 26) (end 1537 39)) (probe (position 1537 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1522 26) (end 1522 39)) (probe (position 1522 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1532 26) (end 1532 39)) (probe (position 1532 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1527 26) (end 1527 39)) (probe (position 1527 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1016 22) (end 1016 37)) (probe (position 1016 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1017 22) (end 1017 38)) (probe (position 1017 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1019 22) (end 1019 71)) (probe (position 1019 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1021 26) (end 1021 59)) (probe (position 1021 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SampledFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1047 38) (end 1047 53)) (probe (position 1047 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario"))) (kind specialization) (ordinal 0) (authored-target "SampledFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1049 47) (end 1049 56)) (probe (position 1049 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint::elapseTime"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::time")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1050 45) (end 1050 58)) (probe (position 1050 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::Scenario::wayPoint::position"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::distance")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1076 49) (end 1076 53)) (probe (position 1076 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleAnalysis::FuelEconomyAnalysisModel::specificGravityOfGasoline"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1141 26) (end 1141 41)) (probe (position 1141 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 925 26) (end 925 45)) (probe (position 925 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 926 24) (end 926 30)) (probe (position 926 24))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 929 29) (end 929 35)) (probe (position 929 29))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 930 31) (end 930 40)) (probe (position 930 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cylinders")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 931 39) (end 931 48)) (probe (position 931 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder1"))) (kind subsetting) (ordinal 0) (authored-target "cylinders")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 932 39) (end 932 48)) (probe (position 932 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder2"))) (kind subsetting) (ordinal 0) (authored-target "cylinders")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 933 39) (end 933 48)) (probe (position 933 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder3"))) (kind subsetting) (ordinal 0) (authored-target "cylinders")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 934 39) (end 934 48)) (probe (position 934 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine4Cyl::cylinder4"))) (kind subsetting) (ordinal 0) (authored-target "cylinders")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 927 31) (end 927 39)) (probe (position 927 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::Engine4Cyl_Variant::engine::cylinders"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 516 31) (end 516 38)) (probe (position 516 31))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 519 40) (end 519 58)) (probe (position 519 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::cargoMass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 518 48) (end 518 64)) (probe (position 518 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::dryMass"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::dryMass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 527 43) (end 527 55)) (probe (position 527 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 529 39) (end 529 43)) (probe (position 529 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 530 41) (end 530 46)) (probe (position 530 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::frontWheels"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 528 42) (end 528 51)) (probe (position 528 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 521 34) (end 521 42)) (probe (position 521 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::fuelTank"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 522 44) (end 522 48)) (probe (position 522 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 517 45) (end 517 58)) (probe (position 517 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::mass"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 520 57) (end 520 66)) (probe (position 520 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::partMasses"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 532 42) (end 532 54)) (probe (position 532 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 534 55) (end 534 59)) (probe (position 534 55))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 533 42) (end 533 51)) (probe (position 533 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 535 38) (end 535 42)) (probe (position 535 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Axle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 536 40) (end 536 45)) (probe (position 536 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_a::PartsTree::vehicle_a::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 537 48) (end 537 56)) (probe (position 537 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "diameter")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 551 26) (end 551 45)) (probe (position 551 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 549 26) (end 549 41)) (probe (position 549 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ShapeItems::Box")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 550 26) (end 550 59)) (probe (position 550 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ParametersOfInterestMetadata::mop")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 742 42) (end 742 59)) (probe (position 742 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::applyParkingBrake"))) (kind featureTyping) (ordinal 0) (authored-target "ApplyParkingBrake")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ApplyParkingBrake")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 741 40) (end 741 55)) (probe (position 741 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::performSelfTest"))) (kind featureTyping) (ordinal 0) (authored-target "PerformSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::PerformSelfTest")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 725 36) (end 725 48)) (probe (position 725 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower"))) (kind featureTyping) (ordinal 0) (authored-target "ProvidePower")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::ProvidePower")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 731 41) (end 731 54)) (probe (position 731 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::amplifyTorque"))) (kind featureTyping) (ordinal 0) (authored-target "AmplifyTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::AmplifyTorque")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 733 44) (end 733 60)) (probe (position 733 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::distributeTorque"))) (kind featureTyping) (ordinal 0) (authored-target "DistributeTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::DistributeTorque")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 726 36) (end 726 43)) (probe (position 726 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ItemDefinitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 726 54) (end 726 60)) (probe (position 726 54))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::fuelCmd"))) (kind redefinition) (ordinal 0) (authored-target "pwrCmd")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 728 42) (end 728 56)) (probe (position 728 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::GenerateTorque")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 732 42) (end 732 56)) (probe (position 732 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::providePower::transferTorque"))) (kind featureTyping) (ordinal 0) (authored-target "TransferTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::TransferTorque")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 743 41) (end 743 57)) (probe (position 743 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::ActionTree::senseTemperature"))) (kind featureTyping) (ordinal 0) (authored-target "SenseTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::ActionDefinitions::SenseTemperature")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 769 38) (end 769 44)) (probe (position 769 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::Driver")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 774 39) (end 774 46)) (probe (position 774 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::DiscreteInteractions::Sequence::part0::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 554 33) (end 554 40)) (probe (position 554 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 575 47) (end 575 50)) (probe (position 575 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 557 40) (end 557 49)) (probe (position 557 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cargoMass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 559 48) (end 559 65)) (probe (position 559 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::avgFuelEconomy"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 680 34) (end 680 42)) (probe (position 680 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy"))) (kind featureTyping) (ordinal 0) (authored-target "BodyAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BodyAssy")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 681 34) (end 681 38)) (probe (position 681 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::bodyAssy::body"))) (kind featureTyping) (ordinal 0) (authored-target "Body")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Body")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 682 42) (end 682 47)) (probe (position 682 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "color")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 665 36) (end 665 46)) (probe (position 665 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft"))) (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Driveshaft")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 666 42) (end 666 51)) (probe (position 666 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 667 41) (end 667 52)) (probe (position 667 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_b")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 668 41) (end 668 52)) (probe (position 668 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_c")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_c")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 556 48) (end 556 55)) (probe (position 556 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::dryMass"))) (kind redefinition) (ordinal 0) (authored-target "dryMass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 645 32) (end 645 38)) (probe (position 645 32))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 647 39) (end 647 47)) (probe (position 647 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::engine::cylinders"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Cylinder")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 588 43) (end 588 55)) (probe (position 588 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 591 39) (end 591 48)) (probe (position 591 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "FrontAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FrontAxle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 592 41) (end 592 46)) (probe (position 592 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::frontWheels"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 589 42) (end 589 51)) (probe (position 589 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 590 41) (end 590 52)) (probe (position 590 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::frontAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 560 38) (end 560 49)) (probe (position 560 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::FuelCmdPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 560 60) (end 560 70)) (probe (position 560 60))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelCmdPort"))) (kind redefinition) (ordinal 0) (authored-target "pwrCmdPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 581 34) (end 581 42)) (probe (position 581 34))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::fuelTank"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::FuelTank")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 582 44) (end 582 48)) (probe (position 582 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 586 44) (end 586 55)) (probe (position 586 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "fuelMassMax")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 595 42) (end 595 54)) (probe (position 595 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 614 42) (end 614 54)) (probe (position 614 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0) (authored-target "Differential")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Differential")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 616 46) (end 616 54)) (probe (position 616 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 617 47) (end 617 55)) (probe (position 617 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::DiffPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 615 45) (end 615 56)) (probe (position 615 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 597 55) (end 597 59)) (probe (position 597 55))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 596 42) (end 596 51)) (probe (position 596 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 620 46) (end 620 54)) (probe (position 620 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0) (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 621 56) (end 621 64)) (probe (position 621 56))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::leftAxleToDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "AxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 622 60) (end 622 78)) (probe (position 622 60))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::leftHalfAxle::shankCompositePort"))) (kind redefinition) (ordinal 0) (authored-target "shankCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 626 47) (end 626 55)) (probe (position 626 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0) (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::HalfAxle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 627 57) (end 627 65)) (probe (position 627 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::rightAxleToDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "AxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::AxlePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 628 60) (end 628 78)) (probe (position 628 60))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearAxle::rightHalfAxle::shankCompositePort"))) (kind redefinition) (ordinal 0) (authored-target "shankCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 600 40) (end 600 45)) (probe (position 600 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 601 48) (end 601 56)) (probe (position 601 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "diameter")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 603 57) (end 603 76)) (probe (position 603 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::lugNutCompositePort"))) (kind redefinition) (ordinal 0) (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 602 49) (end 602 64)) (probe (position 602 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel1::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 607 40) (end 607 45)) (probe (position 607 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 608 48) (end 608 56)) (probe (position 608 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "diameter")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 610 57) (end 610 76)) (probe (position 610 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::lugNutCompositePort"))) (kind redefinition) (ordinal 0) (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 609 49) (end 609 64)) (probe (position 609 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::rearWheel2::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 598 41) (end 598 52)) (probe (position 598 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::rearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_d")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 563 39) (end 563 51)) (probe (position 563 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::setSpeedPort"))) (kind featureTyping) (ordinal 0) (authored-target "SetSpeedPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::SetSpeedPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 677 37) (end 677 48)) (probe (position 677 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::speedSensor"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedSensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SpeedSensor")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 644 38) (end 644 50)) (probe (position 644 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::starterMotor"))) (kind featureTyping) (ordinal 0) (authored-target "StarterMotor")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::StarterMotor")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 660 38) (end 660 50)) (probe (position 660 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 661 42) (end 661 51)) (probe (position 661 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 662 41) (end 662 52)) (probe (position 662 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShaftPort_a")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 671 41) (end 671 56)) (probe (position 671 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleSoftware")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleSoftware")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 672 48) (end 672 65)) (probe (position 672 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::VehicleController")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 674 50) (end 674 66)) (probe (position 674 50))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleSoftware::vehicleController::cruiseController"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::CruiseController")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 564 53) (end 564 70)) (probe (position 564 53))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort"))) (kind redefinition) (ordinal 0) (authored-target "vehicleToRoadPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 565 46) (end 565 61)) (probe (position 565 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort1"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 566 46) (end 566 61)) (probe (position 566 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::PartsTree::vehicle_b::vehicleToRoadPort::wheelToRoadPort2"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 859 30) (end 859 54)) (probe (position 859 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 860 30) (end 860 49)) (probe (position 860 30))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 910 62) (end 910 89)) (probe (position 910 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::drivePowerOutputRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePowerOutputRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::DrivePowerOutputRequirement")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 901 61) (end 901 76)) (probe (position 901 61))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::engineMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 903 44) (end 903 56)) (probe (position 903 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massRequired")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 904 44) (end 904 54)) (probe (position 904 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "massActual")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 906 62) (end 906 89)) (probe (position 906 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::engineSpecification::torqueGenerationRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerationRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::TorqueGenerationRequirement")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 880 52) (end 880 61)) (probe (position 880 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::assumedCargoMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 881 71) (end 881 93)) (probe (position 881 71))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 882 38) (end 882 57)) (probe (position 882 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 885 74) (end 885 96)) (probe (position 885 74))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleFuelEconomyRequirements::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 886 38) (end 886 57)) (probe (position 886 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 866 62) (end 866 77)) (probe (position 866 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::RequirementDefinitions::MassRequirement")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 869 44) (end 869 56)) (probe (position 869 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massRequired")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 870 44) (end 870 54)) (probe (position 870 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "massActual")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 871 50) (end 871 59)) (probe (position 871 50))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassActual"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 872 47) (end 872 56)) (probe (position 872 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b::Requirements::vehicleSpecification::vehicleMassRequirement::fuelMassMax"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 946 26) (end 946 29)) (probe (position 946 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::hub1"))) (kind featureTyping) (ordinal 0) (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 947 48) (end 947 66)) (probe (position 947 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 947 29) (end 947 47)) (probe (position 947 29))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shankCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 941 28) (end 941 33)) (probe (position 941 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy1::wheel1"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 942 48) (end 942 67)) (probe (position 942 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "LugNutCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 942 28) (end 942 47)) (probe (position 942 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 961 26) (end 961 29)) (probe (position 961 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::hub1"))) (kind featureTyping) (ordinal 0) (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 962 48) (end 962 66)) (probe (position 962 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "ShankCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::ShankCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 962 29) (end 962 47)) (probe (position 962 29))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shankCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 956 28) (end 956 33)) (probe (position 956 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy2::wheel1"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 957 48) (end 957 67)) (probe (position 957 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "LugNutCompositePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PortDefinitions::LugNutCompositePort")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 957 28) (end 957 47)) (probe (position 957 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 985 26) (end 985 29)) (probe (position 985 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1"))) (kind featureTyping) (ordinal 0) (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Hub")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 986 48) (end 986 66)) (probe (position 986 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort"))) (kind redefinition) (ordinal 0) (authored-target "shankCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 987 47) (end 987 56)) (probe (position 987 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort"))) (kind redefinition) (ordinal 0) (authored-target "shankPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 992 47) (end 992 56)) (probe (position 992 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort1"))) (kind subsetting) (ordinal 0) (authored-target "shankPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 993 47) (end 993 56)) (probe (position 993 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort2"))) (kind subsetting) (ordinal 0) (authored-target "shankPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 994 47) (end 994 56)) (probe (position 994 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::hub1::shankCompositePort::shankPort3"))) (kind subsetting) (ordinal 0) (authored-target "shankPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 988 42) (end 988 51)) (probe (position 988 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "threadDia")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 989 42) (end 989 53)) (probe (position 989 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "threadPitch")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 990 42) (end 990 53)) (probe (position 990 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "shaftLength")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 974 28) (end 974 33)) (probe (position 974 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Wheel")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 975 49) (end 975 68)) (probe (position 975 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort"))) (kind redefinition) (ordinal 0) (authored-target "lugNutCompositePort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 976 48) (end 976 58)) (probe (position 976 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort"))) (kind redefinition) (ordinal 0) (authored-target "lugNutPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 980 48) (end 980 58)) (probe (position 980 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort1"))) (kind subsetting) (ordinal 0) (authored-target "lugNutPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 981 48) (end 981 58)) (probe (position 981 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort2"))) (kind subsetting) (ordinal 0) (authored-target "lugNutPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 982 48) (end 982 58)) (probe (position 982 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::WheelHubAssemblies::wheelHubAssy3::wheel1::lugNutCompositePort::lugNutPort3"))) (kind subsetting) (ordinal 0) (authored-target "lugNutPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 977 42) (end 977 51)) (probe (position 977 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "threadDia")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 978 42) (end 978 53)) (probe (position 978 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "threadPitch")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 487 41) (end 487 48)) (probe (position 487 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Vehicle")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 495 38) (end 495 54)) (probe (position 495 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::brakingSubsystem"))) (kind featureTyping) (ordinal 0) (authored-target "BrakingSubsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::BrakingSubsystem")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 491 41) (end 491 60)) (probe (position 491 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::electricalGenerator"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricalGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::ElectricalGenerator")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 494 36) (end 494 53)) (probe (position 494 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::steeringSystem"))) (kind featureTyping) (ordinal 0) (authored-target "SteeringSubsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::SteeringSubsystem")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 488 37) (end 488 52)) (probe (position 488 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree::vehicleLogical::torqueGenerator"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::TorqueGenerator")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 501 22) (end 501 63)) (probe (position 501 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleLogicalConfiguration::PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleLogicalConfiguration::PartsTree")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 500 22) (end 500 82)) (probe (position 500 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleConfigurations::VehicleConfiguration_b::PartsTree")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1483 52) (end 1483 64)) (probe (position 1483 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Transmission")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1489 26) (end 1489 54)) (probe (position 1489 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VariationPointDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1492 38) (end 1492 44)) (probe (position 1492 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Engine")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1506 29) (end 1506 36)) (probe (position 1506 29))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::sunroof"))) (kind featureTyping) (ordinal 0) (authored-target "Sunroof")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Definitions::PartDefinitions::Sunroof")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1504 41) (end 1504 60)) (probe (position 1504 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VehiclePartsTree::vehicleFamily::transmissionChoices"))) (kind featureTyping) (ordinal 0) (authored-target "TransmissionChoices")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleSuperSetModel::VariationPointDefinitions::TransmissionChoices")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1203 22) (end 1203 52)) (probe (position 1203 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationCaseDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCaseDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1204 22) (end 1204 43)) (probe (position 1204 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationCases1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationCases1")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1206 22) (end 1206 42)) (probe (position 1206 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationCases")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1207 22) (end 1207 43)) (probe (position 1207 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1202 22) (end 1202 71)) (probe (position 1202 22))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1240 46) (end 1240 55)) (probe (position 1240 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleVerification::VerificationSystem::verificationContext::vehicle_UnitUnderTest"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1569 26) (end 1569 49)) (probe (position 1569 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ViewpointDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewpointDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1570 26) (end 1570 44)) (probe (position 1570 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::Views_Viewpoints::ViewDefinitions")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1571 26) (end 1571 74)) (probe (position 1571 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleConfigurations::VehicleConfiguration_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (qualified-name "SimpleVehicleModel::VehicleConfigurations::VehicleConfiguration_b")))))
  )
  (query (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (range (start 1556 26) (end 1556 34)) (probe (position 1556 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml_v2_spec_annex_a_simple_vehicle_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Views")
      (outcome (status unresolved)))
  )
)
~~~
