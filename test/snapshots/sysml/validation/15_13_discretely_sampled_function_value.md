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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_13_discretely_sampled_function_value.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 1) (end 9 559))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 158))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 1) (end 34 165))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 1) (end 42 280))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 1) (end 50 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 2) (end 54 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 2) (end 67 207))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 2) (end 69 217))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 2) (end 71 225))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "eb87c216cd8ca9cd790f04d4d840ec8a6f169376f131ac185e065389679716be") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (kind "package") (name "15_13-Discretely Sampled Function Value") (declared-name "15_13-Discretely Sampled Function Value") (range (start (line 0) (character 0)) (end (line 0) (character 2825))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 23))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 19))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 18))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 41))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 37))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 24))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 20))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::Array"))) (kind "import") (name "Array") (declared-name "Array") (range (start (line 3) (character 1)) (end (line 3) (character 35))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::Array") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 34))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (kind "attribute def") (name "AscentProfile") (declared-name "AscentProfile") (range (start (line 57) (character 1)) (end (line 57) (character 262))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "SampledFunction") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (kind "attribute def") (name "AscentSample") (declared-name "AscentSample") (range (start (line 58) (character 2)) (end (line 58) (character 159))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (authored (membership (kind Owning)) (relationships (typing (reference "SamplePair") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (range (start (line 62) (character 2)) (end (line 62) (character 49))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample") (range none)) (redefinition (reference "samples") (range (start (line 62) (character 16)) (end (line 62) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (kind "attribute def") (name "MissionElapsedTimeScale") (declared-name "MissionElapsedTimeScale") (range (start (line 9) (character 1)) (end (line 9) (character 559))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeScale") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (kind "attribute") (name "definitionalEpoch") (declared-name "definitionalEpoch") (range (start (line 11) (character 2)) (end (line 11) (character 106))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalEpoch") (range (start (line 11) (character 16)) (end (line 11) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpochInUTC"))) (kind "attribute") (name "definitionalEpochInUTC") (declared-name "definitionalEpochInUTC") (range (start (line 15) (character 2)) (end (line 15) (character 53))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "Iso8601DateTime") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind "attribute") (name "transformation") (declared-name "transformation") (range (start (line 19) (character 2)) (end (line 19) (character 158))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateFramePlacement") (range none)) (redefinition (reference "transformation") (range (start (line 19) (character 16)) (end (line 19) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (range (start (line 10) (character 2)) (end (line 10) (character 15))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unit") (range (start (line 10) (character 2)) (end (line 10) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))) (kind "attribute def") (name "MissionElapsedTimeValue") (declared-name "MissionElapsedTimeValue") (range (start (line 34) (character 1)) (end (line 34) (character 165))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeInstantValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 34) (character 1)) (end (line 34) (character 165))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 39) (character 3)) (end (line 39) (character 19))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRef") (range (start (line 39) (character 3)) (end (line 39) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity"))) (kind "attribute def") (name "PositionAndVelocity") (declared-name "PositionAndVelocity") (range (start (line 52) (character 1)) (end (line 52) (character 145))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 53) (character 2)) (end (line 53) (character 52))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianPosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 54) (character 2)) (end (line 54) (character 52))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianVelocity3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))) (kind "import") (name "SamplePair") (declared-name "SamplePair") (range (start (line 2) (character 1)) (end (line 2) (character 45))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SamplePair") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 44))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))) (kind "import") (name "SampledFunction") (declared-name "SampledFunction") (range (start (line 1) (character 1)) (end (line 1) (character 50))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SampledFunction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 49))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (kind "attribute def") (name "ascentProfile1") (declared-name "ascentProfile1") (range (start (line 65) (character 1)) (end (line 65) (character 787))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "AscentProfile") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::_documentation"))) (kind "documentation") (name "") (range (start (line 65) (character 1)) (end (line 65) (character 787))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample1"))) (kind "attribute") (name "sample1") (declared-name "sample1") (range (start (line 67) (character 2)) (end (line 67) (character 207))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample2"))) (kind "attribute") (name "sample2") (declared-name "sample2") (range (start (line 69) (character 2)) (end (line 69) (character 217))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample3"))) (kind "attribute") (name "sample3") (declared-name "sample3") (range (start (line 71) (character 2)) (end (line 71) (character 225))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (range (start (line 73) (character 2)) (end (line 73) (character 54))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "samples") (range (start (line 73) (character 16)) (end (line 73) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (kind "attribute def") (name "mets") (declared-name "mets") (range (start (line 26) (character 1)) (end (line 26) (character 237))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "MissionElapsedTimeScale") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::_documentation"))) (kind "documentation") (name "") (range (start (line 26) (character 1)) (end (line 26) (character 237))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (kind "attribute") (name "definitionalEpochInUTC") (declared-name "definitionalEpochInUTC") (range (start (line 31) (character 2)) (end (line 31) (character 72))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalEpochInUTC") (range (start (line 31) (character 2)) (end (line 31) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))) (kind "attribute def") (name "spatialCF") (declared-name "spatialCF") (range (start (line 42) (character 1)) (end (line 42) (character 280))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::_documentation"))) (kind "documentation") (name "") (range (start (line 42) (character 1)) (end (line 42) (character 280))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 48) (character 4)) (end (line 48) (character 26))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs") (range (start (line 48) (character 4)) (end (line 48) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::velocityCF"))) (kind "attribute def") (name "velocityCF") (declared-name "velocityCF") (range (start (line 50) (character 1)) (end (line 50) (character 75))) (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianVelocity3dCoordinateFrame") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 4) (character 16)) (end (line 4) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 5) (character 16)) (end (line 5) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 6) (character 16)) (end (line 6) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "Time::*") (range (start (line 7) (character 16)) (end (line 7) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::Array"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::Array") (range (start (line 3) (character 16)) (end (line 3) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind redefinition) (ordinal 0)) (authored-target "samples") (range (start (line 62) (character 16)) (end (line 62) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeScale") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalEpoch") (range (start (line 11) (character 16)) (end (line 11) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpochInUTC"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTime") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFramePlacement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (range (start (line 19) (character 16)) (end (line 19) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (range (start (line 10) (character 2)) (end (line 10) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 39) (character 3)) (end (line 39) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::position"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianPosition3dVector") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dVector") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SamplePair") (range (start (line 2) (character 16)) (end (line 2) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SampledFunction") (range (start (line 1) (character 16)) (end (line 1) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentProfile") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample1"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample2"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample3"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (kind redefinition) (ordinal 0)) (authored-target "samples") (range (start (line 73) (character 16)) (end (line 73) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (kind featureTyping) (ordinal 0)) (authored-target "MissionElapsedTimeScale") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalEpochInUTC") (range (start (line 31) (character 2)) (end (line 31) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 48) (character 4)) (end (line 48) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::velocityCF"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dCoordinateFrame") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::velocityCF")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
