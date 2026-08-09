# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_13-Discretely Sampled Function Value
type=file
~~~
# SOURCE
~~~sysml
package '15_13-Discretely Sampled Function Value' {
	private import SampledFunctions::SampledFunction;
	private import SampledFunctions::SamplePair;
	private import Collections::Array;
	private import ISQ::*;
	private import SI::*;
	private import MeasurementReferences::*;
	private import Time::*;

	attribute def MissionElapsedTimeScale :> TimeScale {
		:>> unit = s;
		attribute :>> definitionalEpoch {
			:>> num = 0;
			:>> definition = "time instant zero at launch";
		}
		attribute definitionalEpochInUTC : Iso8601DateTime;
		
		// Map the definitional epoch (t = 0) of this scale to a reference epoch expressed in UTC
		// This modeled as a 1D coordinate transformation (translation only)
		attribute :>> transformation : CoordinateFramePlacement {
			:>> source = UTC;
			:>> origin = definitionalEpochInUTC;
			:>> basisDirections = 1 [UTC];
		}
  }

	attribute mets: MissionElapsedTimeScale { 
		doc
		/*
		 * Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)
		 */
		:>> definitionalEpochInUTC { :>> val = "2020-08-23T22:42:32.924534Z";}		
	}

	attribute def MissionElapsedTimeValue :> TimeInstantValue {
		doc
		/*
		 * Define scalar quantity value type for mission elapsed time
		 */
	 	:>> mRef = mets; 
	}

	attribute spatialCF: CartesianSpatial3dCoordinateFrame[1] {
		doc
		/*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
	   :>> mRefs = (m, m, m);
	}
	attribute velocityCF: CartesianVelocity3dCoordinateFrame[1] = spatialCF/s;

	attribute def PositionAndVelocity {
		attribute position : CartesianPosition3dVector[1];
		attribute velocity : CartesianVelocity3dVector[1];
	}

	attribute def AscentProfile :> SampledFunction {
		attribute def AscentSample :> SamplePair {
			attribute :>> domainValue: MissionElapsedTimeValue[1];
			attribute :>> rangeValue: PositionAndVelocity[1];
		}
		attribute :>> samples: AscentSample[*] ordered;
	}

	attribute ascentProfile1: AscentProfile {
		doc /* Example ascent profile */
		attribute sample1: AscentSample { :>> domainValue = 0.0 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0, 0, 0) [spatialCF]; :>> velocity = (0, 0, 0) [velocityCF]; } }
		attribute sample2: AscentSample { :>> domainValue = 2.5 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0.01, 0.03, 8.6) [spatialCF]; :>> velocity = (0, 0, 5.5) [velocityCF]; } }
		attribute sample3: AscentSample { :>> domainValue = 5.1 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0.04, 0.12, 18.6) [spatialCF]; :>> velocity = (0.05, 0.03, 25.3) [velocityCF]; } }
		attribute :>> samples = (sample1, sample2, sample3);
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
LineComment,
LineComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_13-Discretely Sampled Function Value''
    (import_decl private 'SampledFunctions::SampledFunction')
    (import_decl private 'SampledFunctions::SamplePair')
    (import_decl private 'Collections::Array')
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'Time::*')
    (attribute_def 'MissionElapsedTimeScale' :> 'TimeScale'
      (default_ref_usage :>> 'unit' value)
      (attribute_usage :>> 'definitionalEpoch'
        (default_ref_usage :>> 'num' value)
        (default_ref_usage :>> 'definition' value))
      (attribute_usage 'definitionalEpochInUTC' : 'Iso8601DateTime')
      (line_comment)
      (line_comment)
      (attribute_usage :>> 'transformation' : 'CoordinateFramePlacement'
        (default_ref_usage :>> 'source' value)
        (default_ref_usage :>> 'origin' value)
        (default_ref_usage :>> 'basisDirections' value)))
    (attribute_usage 'mets' : 'MissionElapsedTimeScale'
      (documentation)
      (default_ref_usage :>> 'definitionalEpochInUTC'
        (default_ref_usage :>> 'val' value)))
    (attribute_def 'MissionElapsedTimeValue' :> 'TimeInstantValue'
      (documentation)
      (default_ref_usage :>> 'mRef' value))
    (attribute_usage 'spatialCF' : 'CartesianSpatial3dCoordinateFrame' multiplicity
      (documentation)
      (default_ref_usage :>> 'mRefs' value))
    (attribute_usage 'velocityCF' : 'CartesianVelocity3dCoordinateFrame' multiplicity value)
    (attribute_def 'PositionAndVelocity'
      (attribute_usage 'position' : 'CartesianPosition3dVector' multiplicity)
      (attribute_usage 'velocity' : 'CartesianVelocity3dVector' multiplicity))
    (attribute_def 'AscentProfile' :> 'SampledFunction'
      (attribute_def 'AscentSample' :> 'SamplePair'
        (attribute_usage :>> 'domainValue' : 'MissionElapsedTimeValue' multiplicity)
        (attribute_usage :>> 'rangeValue' : 'PositionAndVelocity' multiplicity))
      (attribute_usage :>> 'samples' : 'AscentSample' multiplicity ordered))
    (attribute_usage 'ascentProfile1' : 'AscentProfile'
      (documentation)
      (attribute_usage 'sample1' : 'AscentSample'
        (default_ref_usage :>> 'domainValue' value)
        (default_ref_usage :>> 'rangeValue' value)
        (attribute_usage 'pv1' : 'PositionAndVelocity'
          (default_ref_usage :>> 'position' value)
          (default_ref_usage :>> 'velocity' value)))
      (attribute_usage 'sample2' : 'AscentSample'
        (default_ref_usage :>> 'domainValue' value)
        (default_ref_usage :>> 'rangeValue' value)
        (attribute_usage 'pv1' : 'PositionAndVelocity'
          (default_ref_usage :>> 'position' value)
          (default_ref_usage :>> 'velocity' value)))
      (attribute_usage 'sample3' : 'AscentSample'
        (default_ref_usage :>> 'domainValue' value)
        (default_ref_usage :>> 'rangeValue' value)
        (attribute_usage 'pv1' : 'PositionAndVelocity'
          (default_ref_usage :>> 'position' value)
          (default_ref_usage :>> 'velocity' value)))
      (attribute_usage :>> 'samples' value))))
~~~
# FORMAT
~~~sysml
package '15_13-Discretely Sampled Function Value' {
    private import SampledFunctions::SampledFunction;
    private import SampledFunctions::SamplePair;
    private import Collections::Array;
    private import ISQ::*;
    private import SI::*;
    private import MeasurementReferences::*;
    private import Time::*;

    attribute def MissionElapsedTimeScale :> TimeScale {
        :>> unit = s;
        attribute :>> definitionalEpoch {
            :>> num = 0;
            :>> definition = "time instant zero at launch";
        }
        attribute definitionalEpochInUTC : Iso8601DateTime;

        // Map the definitional epoch (t = 0) of this scale to a reference epoch expressed in UTC
        // This modeled as a 1D coordinate transformation (translation only)
        attribute :>> transformation : CoordinateFramePlacement {
            :>> source = UTC;
            :>> origin = definitionalEpochInUTC;
            :>> basisDirections = 1 [UTC];
        }
    }

    attribute mets : MissionElapsedTimeScale {
        doc /*
		 * Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)
		 */
        :>> definitionalEpochInUTC {
            :>> val = "2020-08-23T22:42:32.924534Z";
        }
    }

    attribute def MissionElapsedTimeValue :> TimeInstantValue {
        doc /*
		 * Define scalar quantity value type for mission elapsed time
		 */
        :>> mRef = mets;
    }

    attribute spatialCF : CartesianSpatial3dCoordinateFrame [1] {
        doc /*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
        :>> mRefs = (m, m, m);
    }
    attribute velocityCF : CartesianVelocity3dCoordinateFrame [1] = spatialCF/s;

    attribute def PositionAndVelocity {
        attribute position : CartesianPosition3dVector [1];
        attribute velocity : CartesianVelocity3dVector [1];
    }

    attribute def AscentProfile :> SampledFunction {
        attribute def AscentSample :> SamplePair {
            attribute :>> domainValue : MissionElapsedTimeValue [1];
            attribute :>> rangeValue : PositionAndVelocity [1];
        }
        attribute :>> samples : AscentSample [*] ordered;
    }

    attribute ascentProfile1 : AscentProfile {
        doc /* Example ascent profile */
        attribute sample1 : AscentSample {
            :>> domainValue = 0.0 [mets];
            :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                :>> position = (0, 0, 0) [spatialCF];
                :>> velocity = (0, 0, 0) [velocityCF];
            }
        }
        attribute sample2 : AscentSample {
            :>> domainValue = 2.5 [mets];
            :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                :>> position = (0.01, 0.03, 8.6) [spatialCF];
                :>> velocity = (0, 0, 5.5) [velocityCF];
            }
        }
        attribute sample3 : AscentSample {
            :>> domainValue = 5.1 [mets];
            :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                :>> position = (0.04, 0.12, 18.6) [spatialCF];
                :>> velocity = (0.05, 0.03, 25.3) [velocityCF];
            }
        }
        attribute :>> samples = (sample1, sample2, sample3);
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'TimeScale'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'definitionalEpoch'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'Iso8601DateTime'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'CoordinateFramePlacement'
semantic.unresolved_name 'source'
semantic.unresolved_name 'origin'
semantic.unresolved_name 'basisDirections'
semantic.unresolved_name 'val'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'CartesianVelocity3dCoordinateFrame'
semantic.unresolved_name 'CartesianPosition3dVector'
semantic.unresolved_name 'CartesianVelocity3dVector'
semantic.unresolved_name 'SampledFunction'
semantic.unresolved_name 'SamplePair'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'samples'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'samples'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'TimeScale'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'definitionalEpoch'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'Iso8601DateTime'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'CoordinateFramePlacement'
semantic.unresolved_name 'source'
semantic.unresolved_name 'origin'
semantic.unresolved_name 'basisDirections'
semantic.unresolved_name 'val'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'CartesianVelocity3dCoordinateFrame'
semantic.unresolved_name 'CartesianPosition3dVector'
semantic.unresolved_name 'CartesianVelocity3dVector'
semantic.unresolved_name 'SampledFunction'
semantic.unresolved_name 'SamplePair'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'samples'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'samples'
~~~
# SMG
~~~
(model
  (namespace
    (package '15_13-Discretely Sampled Function Value'
      (membership_import private -> 'SampledFunctions::SampledFunction'[unresolved])
      (membership_import private -> 'SampledFunctions::SamplePair'[unresolved])
      (membership_import private -> 'Collections::Array'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'Time'[unresolved])
      (attribute_def 'MissionElapsedTimeScale' :> 'TimeScale'[unresolved]
        (reference_usage reference :>> 'unit'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'definitionalEpoch'[unresolved]
          (reference_usage reference :>> 'num'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'definition'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'definitionalEpochInUTC' : 'Iso8601DateTime'[unresolved])
        (attribute_usage composite :>> 'transformation'[unresolved] : 'CoordinateFramePlacement'[unresolved]
          (reference_usage reference :>> 'source'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'origin'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'basisDirections'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'mets' : '15_13-Discretely Sampled Function Value::MissionElapsedTimeScale'[attribute_def]
        (documentation)
        (reference_usage reference :>> '15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpochInUTC'[attribute_usage]
          (reference_usage reference :>> 'val'[unresolved]
            (feature_value (=)))))
      (attribute_def 'MissionElapsedTimeValue' :> 'TimeInstantValue'[unresolved]
        (documentation)
        (reference_usage reference :>> 'mRef'[unresolved]
          (feature_value (=))))
      (attribute_usage 'spatialCF' : 'CartesianSpatial3dCoordinateFrame'[unresolved]
        (multiplicity_range [1])
        (documentation)
        (reference_usage reference :>> 'mRefs'[unresolved]
          (feature_value (=))))
      (attribute_usage 'velocityCF' : 'CartesianVelocity3dCoordinateFrame'[unresolved]
        (multiplicity_range [1])
        (feature_value (=)))
      (attribute_def 'PositionAndVelocity'
        (attribute_usage composite 'position' : 'CartesianPosition3dVector'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'velocity' : 'CartesianVelocity3dVector'[unresolved]
          (multiplicity_range [1])))
      (attribute_def 'AscentProfile' :> 'SampledFunction'[unresolved]
        (attribute_def 'AscentSample' :> 'SamplePair'[unresolved]
          (attribute_usage composite :>> 'domainValue'[unresolved] : '15_13-Discretely Sampled Function Value::MissionElapsedTimeValue'[attribute_def]
            (multiplicity_range [1]))
          (attribute_usage composite :>> 'rangeValue'[unresolved] : '15_13-Discretely Sampled Function Value::PositionAndVelocity'[attribute_def]
            (multiplicity_range [1])))
        (attribute_usage composite ordered :>> 'samples'[unresolved] : '15_13-Discretely Sampled Function Value::AscentProfile::AscentSample'[attribute_def]
          (multiplicity_range [*])))
      (attribute_usage 'ascentProfile1' : '15_13-Discretely Sampled Function Value::AscentProfile'[attribute_def]
        (documentation)
        (attribute_usage composite 'sample1' : '15_13-Discretely Sampled Function Value::AscentProfile::AscentSample'[attribute_def]
          (reference_usage reference :>> 'domainValue'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'rangeValue'[unresolved]
            (feature_value (=)))
          (attribute_usage composite 'pv1' : '15_13-Discretely Sampled Function Value::PositionAndVelocity'[attribute_def]
            (reference_usage reference :>> '15_13-Discretely Sampled Function Value::PositionAndVelocity::position'[attribute_usage]
              (feature_value (=)))
            (reference_usage reference :>> '15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity'[attribute_usage]
              (feature_value (=)))))
        (attribute_usage composite 'sample2' : '15_13-Discretely Sampled Function Value::AscentProfile::AscentSample'[attribute_def]
          (reference_usage reference :>> 'domainValue'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'rangeValue'[unresolved]
            (feature_value (=)))
          (attribute_usage composite 'pv1' : '15_13-Discretely Sampled Function Value::PositionAndVelocity'[attribute_def]
            (reference_usage reference :>> '15_13-Discretely Sampled Function Value::PositionAndVelocity::position'[attribute_usage]
              (feature_value (=)))
            (reference_usage reference :>> '15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity'[attribute_usage]
              (feature_value (=)))))
        (attribute_usage composite 'sample3' : '15_13-Discretely Sampled Function Value::AscentProfile::AscentSample'[attribute_def]
          (reference_usage reference :>> 'domainValue'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'rangeValue'[unresolved]
            (feature_value (=)))
          (attribute_usage composite 'pv1' : '15_13-Discretely Sampled Function Value::PositionAndVelocity'[attribute_def]
            (reference_usage reference :>> '15_13-Discretely Sampled Function Value::PositionAndVelocity::position'[attribute_usage]
              (feature_value (=)))
            (reference_usage reference :>> '15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity'[attribute_usage]
              (feature_value (=)))))
        (attribute_usage composite :>> 'samples'[unresolved]
          (feature_value (=)))))))
~~~
