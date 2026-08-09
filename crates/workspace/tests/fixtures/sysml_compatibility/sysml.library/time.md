# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/Time
type=file
~~~
# SOURCE
~~~sysml
standard library package Time {
	doc
	/*
	 * This package specifies concepts to support time-related quantities and metrology, beyond 
	 * the quantities duration and time as defined in [ISO 80000-3]. Representations of the 
	 * Gregorian calendar date and time of day as specified by the [ISO 8601-1] standard are used.
	 */

	private import Occurrences::Occurrence;
	private import ScalarValues::Real;
	private import ScalarValues::Integer;
	private import ScalarValues::Natural;
	private import ScalarValues::String;
	private import Quantities::ScalarQuantityValue;
	private import Quantities::scalarQuantities;
    private import MeasurementReferences::*;
    public import ISQBase::DurationValue;
    public import ISQBase::DurationUnit;
    public import ISQBase::duration;
    public import ISQSpaceTime::TimeValue;
    public import ISQSpaceTime::TimeUnit;
    public import ISQSpaceTime::time;
    
    part universalClock : Clock[1] :> Clocks::universalClock {
   	    doc
	    /*
	     * universalClock is a single Clock that can be used as a default universal time reference.
	     */
    }

	part def Clock :> Clocks::Clock {
		doc
		/*
		 * A Clock provides a currentTime as a TimeInstantValue that advances montonically over its lifetime.
		 */
	
		attribute :>> currentTime : TimeInstantValue;
	}
	
	calc def TimeOf :> Clocks::TimeOf {
		doc
		/*
		 * TimeOf returns a TimeInstantValue for a given Occurrence relative to a given Clock. This TimeInstantValue is the 
		 * time of the start of the Occurrence, which is considered to be synchronized with the snapshot of the Clock with a 
		 * currentTime equal to the returned timeInstant.
		 */
	
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return timeInstant : TimeInstantValue[1];
	}

	calc def DurationOf :> Clocks::DurationOf {
		doc
		/*
		 * DurationOf returns the duration of a given Occurrence relative to a given Clock, which is equal to the TimeOf 
		 * the end snapshot of the Occurrence minus the TimeOf its start snapshot.
		 */
	
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return duration : DurationValue;
	}
	
    attribute def TimeScale :> IntervalScale {
		doc
		/*
		 * Generic time scale to express a time instant, including a textual definition of the meaning of zero time instant value
		 * 
		 * Attribute definitionalEpoch captures the specification of the time instant with value zero, also known as the (reference) epoch.
		 */
	
		attribute :>> unit: DurationUnit[1];
		attribute definitionalEpoch: DefinitionalQuantityValue[1];
		attribute :>> definitionalQuantityValues = definitionalEpoch;
    }

    attribute def TimeInstantValue :> ScalarQuantityValue {
		doc
		/*
		 * Representation of a time instant quantity
		 *
		 * Also known as instant (of time), or, point in time.
		 */
	
        attribute :>> num: Real[1];
        attribute :>> mRef: TimeScale[1];
    }
    attribute timeInstant: TimeInstantValue :> scalarQuantities;

	abstract attribute def DateTime :> TimeInstantValue {
		doc
		/*
		 * Generic representation of a time instant as a calendar date and time of day
		 */
	}

	abstract attribute def Date :> TimeInstantValue {
		doc
		/*
		 * Generic representation of a time instant as a calendar date
		 */
	}

	abstract attribute def TimeOfDay :> TimeInstantValue {
		doc
		/*
		 * Generic representation of a time instant as a time of day
		 */
	}

	attribute <UTC> 'Coordinated Universal Time' : TimeScale {
		doc
		/*
		 * Representation of the Coordinated Universal Time (UTC) time scale
		 *
		 * UTC is the primary time standard by which the world regulates clocks and time. It is within about 1 second of mean solar time
		 * at 0° longitude and is not adjusted for daylight saving time.
		 * UTC is obtained from International Atomic Time (TAI) by the insertion of leap seconds according to the advice of
		 * the International Earth Rotation and Reference Systems Service (IERS) to ensure approximate agreement
		 * with the time derived from the rotation of the Earth.
		 *
		 * References:
		 * ITU-R TF.460-6 (see https://www.itu.int/rec/R-REC-TF.460/en)
		 * BIPM technical services: Time Metrology (see https://www.bipm.org/en/time-metrology)
		 *
		 * Introductions:
		 * For UTC see https://en.wikipedia.org/wiki/Coordinated_Universal_Time
		 * For TAI see https://en.wikipedia.org/wiki/International_Atomic_Time
		 */
	
		attribute :>> unit = SI::s;
		attribute :>> definitionalEpoch: DefinitionalQuantityValue { :>> num = 0; :>> definition = "UTC epoch at 1 January 1958 at 0 hour 0 minute 0 second"; }
	}

	attribute def UtcTimeInstantValue :> DateTime { 
		:>> mRef = UTC {
			doc
			/*
			 * Representation of a time instant expressed on the Coordinated Universal Time (UTC) time scale
			 */
		} 
	}
	attribute utcTimeInstant: UtcTimeInstantValue :> timeInstant;

	/*
	 * Representations of a Gregorian calendar date and time of day as specified by the ISO 8601-1 standard.
	 *
	 * As explained in ISO 8601-1 clause 4.2.1:
	 * ISO 8601-1 uses the Gregorian calendar for the identification of calendar days.
	 *
	 * The Gregorian calendar provides a time scale consisting of a series of contiguous calendar years,
	 * each identified by a year number represented by an integer, greater than that of the
	 * immediately preceding calendar year by 1. ISO 8601-1 allows the identification of calendar years
	 * by their year number for years both before and after the introduction of the Gregorian calendar.
	 *
	 * The Gregorian calendar distinguishes common years of 365 consecutive calendar days and leap years
	 * of 366 consecutive calendar days.
	 *
	 * In the Gregorian calendar each calendar year is divided into 12 sequential calendar months,
	 * each consisting of a specific number of calendar days in the range 28 to 31. Usage of the Gregorian calendar
	 * for identifying dates preceding its introduction (15 October 1582) should only be by mutual agreement
	 * of the communicating partners.
	 *
	 * Reference: ISO 8601-1:2019 (First edition)
	 * "Date and time — Representations for information interchange — Part 1: Basic rules"
	 * (see https://www.iso.org/standard/70907.html)
	 */

	attribute def Iso8601DateTimeEncoding :> String {
	    doc
	    /*
	     * Extended string encoding of an ISO 8601-1 date and time
	     *
	     * The format of the string must comply with the following EBNF production:
	     * ['+' | '-'] YYYY '-' MM '-' DD 'T' hh ':' mm ':' ss ['.' fff [fff]] ('Z' | timezoneOffset )
	     * where:
	     *   YYYY is 4-or-more-digit year number, which can be negative for years before 0000;
	     *   MM is 2-digit month in year number, in which 01 is January, 02 is February, ..., 12 is December;
	     *   DD is 2-digit day in month number in range 01 to 28, 29, 30, 31 depending on month and leap year;
	     *   hh is 2-digit hour in day number in range 00 to 23;
	     *   mm is 2-digit minute in hour in range 00 to 59;
	     *   ss is 2-digit second in minute in range 00 to 60, in  in case of leap second;
	     *   ['.' fff [fff]] is an optional 3-digit millisecond or 6-digit microsecond fraction;
	     *   timezoneOffset is ('+' | '-') hhOffset ':' mmOffset, denoting the local timezone hour and minute offset w.r.t. UTC,
	     *   in which '+' specifies an offset ahead of UTC and '-' specifies an offset behind UTC;
	     *
	     * Note 1: All components are expressed with leading zeros.
	     * Note 2: 'Z' instead of timezoneOffset denotes a UTC time, i.e. zero time offset.
	     * Note 3: The ss value may only be 60 when a leap second is inserted.
	     *
	     * Examples of such a date and time value are:
	     * 2021-08-30T12:30:24Z (UTC date and time with second precision)
	     * 2018-01-23T23:14:44.304827Z (UTC date and time with microsecond precision)
	     * 1969-07-20T20:17:00Z (UTC date and time with second precision)
	     * 1969-07-20T15:17:00-05:00 (local date and time with second precision for a timezone 5 hour behind UTC)
	     * 1969-07-20T22:17:00+02:00 (local date and time with second precision for a timezone 2 hour ahead of UTC)
	     */
    }

    attribute def Iso8601DateTime :> UtcTimeInstantValue {
		doc
		/*
	     * Representation of an ISO 8601-1 date and time in extended string format
		 */
	
    	attribute val: Iso8601DateTimeEncoding;
    	attribute :>> num = getElapsedUtcTime(val);
    	private calc getElapsedUtcTime {
    		in iso8601DateTime: Iso8601DateTimeEncoding;
    		/* Return the number of seconds elapsed since the UTC epoch. 
    		 * Can be negative when the date and time is earlier than the epoch.
    		 */
    		return : Real;
    	}
    }

    attribute def Iso8601DateTimeStructure :> UtcTimeInstantValue {
		doc
		/*
	     * Representation of an ISO 8601 date and time with explicit date and time component attributes
	     *
	     * The total time offset is equal to the summation of hourOffset and minuteOffset.
		 */
	
    	attribute year: Integer;
    	attribute month: Natural;
    	attribute day: Natural;
    	attribute hour: Natural;
    	attribute minute: Natural;
    	attribute second: Natural;
    	attribute microsecond: Natural;
    	attribute hourOffset: Integer;
    	attribute minuteOffset: Integer;
    	attribute :>> num = getElapsedUtcTime(year, month, day, hour, minute, second, microsecond, hourOffset, minuteOffset);
    	private calc getElapsedUtcTime {
    		in year: Integer; 
    		in month: Natural; 
    		in day: Natural;
    		in hour: Natural;
    		in minute: Natural;
    		in second: Natural;
    		in microsecond: Natural;
    		in hourOffset: Integer;
    		in minuteOffest: Integer;
    		return : Real;
    	}
    }

	calc convertIso8601DateTimeToStructure {
	    doc
	    /*
		 * Calculation to convert an ISO 8601 date and time instant from string to component structure representation
	     */
    
		in iso8601DateTime: Iso8601DateTime;
		/* Parse ISO 8601 string encoding to date and time components */
		return : Iso8601DateTimeStructure;
	}

	calc convertIso8601StructureToDateTime {
		doc
		/*
		 * Calculation to convert an ISO 8601 date and time instant from component structure to string representation
		 */
	
		in iso8601DateTimeStructure: Iso8601DateTimeStructure;
		attribute x: Iso8601DateTime;
		/* Concatenate ISO 8601 date and time components to string 
		 *     year-month-dayThour:minute:second±hourOffset:minuteOffset
		 */
		return : Iso8601DateTime;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Clocks::universalClock'
semantic.unresolved_name 'Clocks::Clock'
semantic.unresolved_name 'currentTime'
semantic.unresolved_name 'Clocks::TimeOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Clocks::DurationOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'IntervalScale'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'definitionalQuantityValues'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'String'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Clocks::universalClock'
semantic.unresolved_name 'Clocks::Clock'
semantic.unresolved_name 'currentTime'
semantic.unresolved_name 'Clocks::TimeOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Clocks::DurationOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'IntervalScale'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'definitionalQuantityValues'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'String'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwPrivate,KwCalc,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
RegularComment,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwPrivate,KwCalc,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwCalc,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
RegularComment,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
RegularComment,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Time'
    (documentation)
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ScalarValues::Integer')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'ScalarValues::String')
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'Quantities::scalarQuantities')
    (import_decl private 'MeasurementReferences::*')
    (import_decl public 'ISQBase::DurationValue')
    (import_decl public 'ISQBase::DurationUnit')
    (import_decl public 'ISQBase::duration')
    (import_decl public 'ISQSpaceTime::TimeValue')
    (import_decl public 'ISQSpaceTime::TimeUnit')
    (import_decl public 'ISQSpaceTime::time')
    (part_usage 'universalClock' : 'Clock' :> 'Clocks::universalClock' multiplicity
      (documentation))
    (part_def 'Clock' :> 'Clocks::Clock'
      (documentation)
      (attribute_usage :>> 'currentTime' : 'TimeInstantValue'))
    (calc_def 'TimeOf' :> 'Clocks::TimeOf'
      (documentation)
      (default_ref_usage in 'o' : 'Occurrence' multiplicity)
      (default_ref_usage in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (calc_def 'DurationOf' :> 'Clocks::DurationOf'
      (documentation)
      (default_ref_usage in 'o' : 'Occurrence' multiplicity)
      (default_ref_usage in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (attribute_def 'TimeScale' :> 'IntervalScale'
      (documentation)
      (attribute_usage :>> 'unit' : 'DurationUnit' multiplicity)
      (attribute_usage 'definitionalEpoch' : 'DefinitionalQuantityValue' multiplicity)
      (attribute_usage :>> 'definitionalQuantityValues' value))
    (attribute_def 'TimeInstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real' multiplicity)
      (attribute_usage :>> 'mRef' : 'TimeScale' multiplicity))
    (attribute_usage 'timeInstant' : 'TimeInstantValue' :> 'scalarQuantities')
    (attribute_def abstract 'DateTime' :> 'TimeInstantValue'
      (documentation))
    (attribute_def abstract 'Date' :> 'TimeInstantValue'
      (documentation))
    (attribute_def abstract 'TimeOfDay' :> 'TimeInstantValue'
      (documentation))
    (attribute_usage ''Coordinated Universal Time'' : 'TimeScale'
      (documentation)
      (attribute_usage :>> 'unit' value)
      (attribute_usage :>> 'definitionalEpoch' : 'DefinitionalQuantityValue'
        (default_ref_usage :>> 'num' value)
        (default_ref_usage :>> 'definition' value)))
    (attribute_def 'UtcTimeInstantValue' :> 'DateTime'
      (default_ref_usage :>> 'mRef' value
        (documentation)))
    (attribute_usage 'utcTimeInstant' : 'UtcTimeInstantValue' :> 'timeInstant')
    (comment)
    (attribute_def 'Iso8601DateTimeEncoding' :> 'String'
      (documentation))
    (attribute_def 'Iso8601DateTime' :> 'UtcTimeInstantValue'
      (documentation)
      (attribute_usage 'val' : 'Iso8601DateTimeEncoding')
      (attribute_usage :>> 'num' value)
      (calc_usage private 'getElapsedUtcTime'
        (default_ref_usage in 'iso8601DateTime' : 'Iso8601DateTimeEncoding')
        (comment)
        (return_member)))
    (attribute_def 'Iso8601DateTimeStructure' :> 'UtcTimeInstantValue'
      (documentation)
      (attribute_usage 'year' : 'Integer')
      (attribute_usage 'month' : 'Natural')
      (attribute_usage 'day' : 'Natural')
      (attribute_usage 'hour' : 'Natural')
      (attribute_usage 'minute' : 'Natural')
      (attribute_usage 'second' : 'Natural')
      (attribute_usage 'microsecond' : 'Natural')
      (attribute_usage 'hourOffset' : 'Integer')
      (attribute_usage 'minuteOffset' : 'Integer')
      (attribute_usage :>> 'num' value)
      (calc_usage private 'getElapsedUtcTime'
        (default_ref_usage in 'year' : 'Integer')
        (default_ref_usage in 'month' : 'Natural')
        (default_ref_usage in 'day' : 'Natural')
        (default_ref_usage in 'hour' : 'Natural')
        (default_ref_usage in 'minute' : 'Natural')
        (default_ref_usage in 'second' : 'Natural')
        (default_ref_usage in 'microsecond' : 'Natural')
        (default_ref_usage in 'hourOffset' : 'Integer')
        (default_ref_usage in 'minuteOffest' : 'Integer')
        (return_member)))
    (calc_usage 'convertIso8601DateTimeToStructure'
      (documentation)
      (default_ref_usage in 'iso8601DateTime' : 'Iso8601DateTime')
      (comment)
      (return_member))
    (calc_usage 'convertIso8601StructureToDateTime'
      (documentation)
      (default_ref_usage in 'iso8601DateTimeStructure' : 'Iso8601DateTimeStructure')
      (attribute_usage 'x' : 'Iso8601DateTime')
      (comment)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package Time {
    doc /*
	 * This package specifies concepts to support time-related quantities and metrology, beyond 
	 * the quantities duration and time as defined in [ISO 80000-3]. Representations of the 
	 * Gregorian calendar date and time of day as specified by the [ISO 8601-1] standard are used.
	 */

    private import Occurrences::Occurrence;
    private import ScalarValues::Real;
    private import ScalarValues::Integer;
    private import ScalarValues::Natural;
    private import ScalarValues::String;
    private import Quantities::ScalarQuantityValue;
    private import Quantities::scalarQuantities;
    private import MeasurementReferences::*;
    public import ISQBase::DurationValue;
    public import ISQBase::DurationUnit;
    public import ISQBase::duration;
    public import ISQSpaceTime::TimeValue;
    public import ISQSpaceTime::TimeUnit;
    public import ISQSpaceTime::time;

    part universalClock : Clock :> Clocks::universalClock [1] {
        doc /*
	     * universalClock is a single Clock that can be used as a default universal time reference.
	     */
    }

    part def Clock :> Clocks::Clock {
        doc /*
		 * A Clock provides a currentTime as a TimeInstantValue that advances montonically over its lifetime.
		 */

        attribute :>> currentTime : TimeInstantValue;
    }

    calc def TimeOf :> Clocks::TimeOf {
        doc /*
		 * TimeOf returns a TimeInstantValue for a given Occurrence relative to a given Clock. This TimeInstantValue is the 
		 * time of the start of the Occurrence, which is considered to be synchronized with the snapshot of the Clock with a 
		 * currentTime equal to the returned timeInstant.
		 */

        in o : Occurrence [1];
        in clock : Clock [1] default = localClock;
        return timeInstant : TimeInstantValue[1];
    }

    calc def DurationOf :> Clocks::DurationOf {
        doc /*
		 * DurationOf returns the duration of a given Occurrence relative to a given Clock, which is equal to the TimeOf 
		 * the end snapshot of the Occurrence minus the TimeOf its start snapshot.
		 */

        in o : Occurrence [1];
        in clock : Clock [1] default = localClock;
        return duration : DurationValue;
    }

    attribute def TimeScale :> IntervalScale {
        doc /*
		 * Generic time scale to express a time instant, including a textual definition of the meaning of zero time instant value
		 * 
		 * Attribute definitionalEpoch captures the specification of the time instant with value zero, also known as the (reference) epoch.
		 */

        attribute :>> unit : DurationUnit [1];
        attribute definitionalEpoch : DefinitionalQuantityValue [1];
        attribute :>> definitionalQuantityValues = definitionalEpoch;
    }

    attribute def TimeInstantValue :> ScalarQuantityValue {
        doc /*
		 * Representation of a time instant quantity
		 *
		 * Also known as instant (of time), or, point in time.
		 */

        attribute :>> num : Real [1];
        attribute :>> mRef : TimeScale [1];
    }
    attribute timeInstant : TimeInstantValue :> scalarQuantities;

    abstract attribute def DateTime :> TimeInstantValue {
        doc /*
		 * Generic representation of a time instant as a calendar date and time of day
		 */
    }

    abstract attribute def Date :> TimeInstantValue {
        doc /*
		 * Generic representation of a time instant as a calendar date
		 */
    }

    abstract attribute def TimeOfDay :> TimeInstantValue {
        doc /*
		 * Generic representation of a time instant as a time of day
		 */
    }

    attribute <UTC> 'Coordinated Universal Time' : TimeScale {
        doc /*
		 * Representation of the Coordinated Universal Time (UTC) time scale
		 *
		 * UTC is the primary time standard by which the world regulates clocks and time. It is within about 1 second of mean solar time
		 * at 0° longitude and is not adjusted for daylight saving time.
		 * UTC is obtained from International Atomic Time (TAI) by the insertion of leap seconds according to the advice of
		 * the International Earth Rotation and Reference Systems Service (IERS) to ensure approximate agreement
		 * with the time derived from the rotation of the Earth.
		 *
		 * References:
		 * ITU-R TF.460-6 (see https://www.itu.int/rec/R-REC-TF.460/en)
		 * BIPM technical services: Time Metrology (see https://www.bipm.org/en/time-metrology)
		 *
		 * Introductions:
		 * For UTC see https://en.wikipedia.org/wiki/Coordinated_Universal_Time
		 * For TAI see https://en.wikipedia.org/wiki/International_Atomic_Time
		 */

        attribute :>> unit = SI::s;
        attribute :>> definitionalEpoch : DefinitionalQuantityValue {
            :>> num = 0;
            :>> definition = "UTC epoch at 1 January 1958 at 0 hour 0 minute 0 second";
        }
    }

    attribute def UtcTimeInstantValue :> DateTime {
        :>> mRef = UTC {
            doc /*
			 * Representation of a time instant expressed on the Coordinated Universal Time (UTC) time scale
			 */
        }
    }
    attribute utcTimeInstant : UtcTimeInstantValue :> timeInstant;

    /*
	 * Representations of a Gregorian calendar date and time of day as specified by the ISO 8601-1 standard.
	 *
	 * As explained in ISO 8601-1 clause 4.2.1:
	 * ISO 8601-1 uses the Gregorian calendar for the identification of calendar days.
	 *
	 * The Gregorian calendar provides a time scale consisting of a series of contiguous calendar years,
	 * each identified by a year number represented by an integer, greater than that of the
	 * immediately preceding calendar year by 1. ISO 8601-1 allows the identification of calendar years
	 * by their year number for years both before and after the introduction of the Gregorian calendar.
	 *
	 * The Gregorian calendar distinguishes common years of 365 consecutive calendar days and leap years
	 * of 366 consecutive calendar days.
	 *
	 * In the Gregorian calendar each calendar year is divided into 12 sequential calendar months,
	 * each consisting of a specific number of calendar days in the range 28 to 31. Usage of the Gregorian calendar
	 * for identifying dates preceding its introduction (15 October 1582) should only be by mutual agreement
	 * of the communicating partners.
	 *
	 * Reference: ISO 8601-1:2019 (First edition)
	 * "Date and time — Representations for information interchange — Part 1: Basic rules"
	 * (see https://www.iso.org/standard/70907.html)
	 */

    attribute def Iso8601DateTimeEncoding :> String {
        doc /*
	     * Extended string encoding of an ISO 8601-1 date and time
	     *
	     * The format of the string must comply with the following EBNF production:
	     * ['+' | '-'] YYYY '-' MM '-' DD 'T' hh ':' mm ':' ss ['.' fff [fff]] ('Z' | timezoneOffset )
	     * where:
	     *   YYYY is 4-or-more-digit year number, which can be negative for years before 0000;
	     *   MM is 2-digit month in year number, in which 01 is January, 02 is February, ..., 12 is December;
	     *   DD is 2-digit day in month number in range 01 to 28, 29, 30, 31 depending on month and leap year;
	     *   hh is 2-digit hour in day number in range 00 to 23;
	     *   mm is 2-digit minute in hour in range 00 to 59;
	     *   ss is 2-digit second in minute in range 00 to 60, in  in case of leap second;
	     *   ['.' fff [fff]] is an optional 3-digit millisecond or 6-digit microsecond fraction;
	     *   timezoneOffset is ('+' | '-') hhOffset ':' mmOffset, denoting the local timezone hour and minute offset w.r.t. UTC,
	     *   in which '+' specifies an offset ahead of UTC and '-' specifies an offset behind UTC;
	     *
	     * Note 1: All components are expressed with leading zeros.
	     * Note 2: 'Z' instead of timezoneOffset denotes a UTC time, i.e. zero time offset.
	     * Note 3: The ss value may only be 60 when a leap second is inserted.
	     *
	     * Examples of such a date and time value are:
	     * 2021-08-30T12:30:24Z (UTC date and time with second precision)
	     * 2018-01-23T23:14:44.304827Z (UTC date and time with microsecond precision)
	     * 1969-07-20T20:17:00Z (UTC date and time with second precision)
	     * 1969-07-20T15:17:00-05:00 (local date and time with second precision for a timezone 5 hour behind UTC)
	     * 1969-07-20T22:17:00+02:00 (local date and time with second precision for a timezone 2 hour ahead of UTC)
	     */
    }

    attribute def Iso8601DateTime :> UtcTimeInstantValue {
        doc /*
	     * Representation of an ISO 8601-1 date and time in extended string format
		 */

        attribute val : Iso8601DateTimeEncoding;
        attribute :>> num = getElapsedUtcTime(val);
        private calc getElapsedUtcTime {
            in iso8601DateTime : Iso8601DateTimeEncoding;
            /* Return the number of seconds elapsed since the UTC epoch. 
    		 * Can be negative when the date and time is earlier than the epoch.
    		 */
            return : Real;
        }
    }

    attribute def Iso8601DateTimeStructure :> UtcTimeInstantValue {
        doc /*
	     * Representation of an ISO 8601 date and time with explicit date and time component attributes
	     *
	     * The total time offset is equal to the summation of hourOffset and minuteOffset.
		 */

        attribute year : Integer;
        attribute month : Natural;
        attribute day : Natural;
        attribute hour : Natural;
        attribute minute : Natural;
        attribute second : Natural;
        attribute microsecond : Natural;
        attribute hourOffset : Integer;
        attribute minuteOffset : Integer;
        attribute :>> num = getElapsedUtcTime(year, month, day, hour, minute, second, microsecond, hourOffset, minuteOffset);
        private calc getElapsedUtcTime {
            in year : Integer;
            in month : Natural;
            in day : Natural;
            in hour : Natural;
            in minute : Natural;
            in second : Natural;
            in microsecond : Natural;
            in hourOffset : Integer;
            in minuteOffest : Integer;
            return : Real;
        }
    }

    calc convertIso8601DateTimeToStructure {
        doc /*
		 * Calculation to convert an ISO 8601 date and time instant from string to component structure representation
	     */

        in iso8601DateTime : Iso8601DateTime;
        /* Parse ISO 8601 string encoding to date and time components */
        return : Iso8601DateTimeStructure;
    }

    calc convertIso8601StructureToDateTime {
        doc /*
		 * Calculation to convert an ISO 8601 date and time instant from component structure to string representation
		 */

        in iso8601DateTimeStructure : Iso8601DateTimeStructure;
        attribute x : Iso8601DateTime;
        /* Concatenate ISO 8601 date and time components to string 
		 *     year-month-dayThour:minute:second±hourOffset:minuteOffset
		 */
        return : Iso8601DateTime;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Time'
      (documentation)
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'ScalarValues::Integer'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (membership_import private -> 'Quantities::scalarQuantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (membership_import public -> 'ISQBase::DurationValue'[unresolved])
      (membership_import public -> 'ISQBase::DurationUnit'[unresolved])
      (membership_import public -> 'ISQBase::duration'[unresolved])
      (membership_import public -> 'ISQSpaceTime::TimeValue'[unresolved])
      (membership_import public -> 'ISQSpaceTime::TimeUnit'[unresolved])
      (membership_import public -> 'ISQSpaceTime::time'[unresolved])
      (part_usage 'universalClock' : 'Time::Clock'[part_def] :> 'Clocks::universalClock'[unresolved]
        (multiplicity_range [1])
        (documentation))
      (part_def 'Clock' :> 'Clocks::Clock'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'currentTime'[unresolved] : 'Time::TimeInstantValue'[attribute_def]))
      (calculation_def 'TimeOf' :> 'Clocks::TimeOf'[unresolved]
        (documentation)
        (reference_usage in reference 'o' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'clock' : 'Time::Clock'[part_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'timeInstant' : 'Time::TimeInstantValue'[attribute_def]
            (multiplicity_range [1]))))
      (calculation_def 'DurationOf' :> 'Clocks::DurationOf'[unresolved]
        (documentation)
        (reference_usage in reference 'o' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'clock' : 'Time::Clock'[part_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'duration' : 'DurationValue'[unresolved])))
      (attribute_def 'TimeScale' :> 'IntervalScale'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'unit'[unresolved] : 'DurationUnit'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'definitionalEpoch' : 'DefinitionalQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'definitionalQuantityValues'[unresolved]
          (feature_value (=))))
      (attribute_def 'TimeInstantValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'Time::TimeScale'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'timeInstant' : 'Time::TimeInstantValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def abstract 'DateTime' :> 'Time::TimeInstantValue'[attribute_def]
        (documentation))
      (attribute_def abstract 'Date' :> 'Time::TimeInstantValue'[attribute_def]
        (documentation))
      (attribute_def abstract 'TimeOfDay' :> 'Time::TimeInstantValue'[attribute_def]
        (documentation))
      (attribute_usage 'Coordinated Universal Time' : 'Time::TimeScale'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'unit'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'Time::TimeScale::definitionalEpoch'[attribute_usage] : 'DefinitionalQuantityValue'[unresolved]
          (reference_usage reference :>> 'num'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'definition'[unresolved]
            (feature_value (=)))))
      (attribute_def 'UtcTimeInstantValue' :> 'Time::DateTime'[attribute_def]
        (reference_usage reference :>> 'mRef'[unresolved]
          (feature_value (=))
          (documentation)))
      (attribute_usage 'utcTimeInstant' : 'Time::UtcTimeInstantValue'[attribute_def] :> 'Time::timeInstant'[attribute_usage])
      (attribute_def 'Iso8601DateTimeEncoding' :> 'String'[unresolved]
        (documentation))
      (attribute_def 'Iso8601DateTime' :> 'Time::UtcTimeInstantValue'[attribute_def]
        (documentation)
        (attribute_usage composite 'val' : 'Time::Iso8601DateTimeEncoding'[attribute_def])
        (attribute_usage composite :>> 'num'[unresolved]
          (feature_value (=)))
        (calculation_usage composite 'getElapsedUtcTime'
          (reference_usage in reference 'iso8601DateTime' : 'Time::Iso8601DateTimeEncoding'[attribute_def])
          (return_parameter_membership
            (feature_def out : 'Real'[unresolved]))))
      (attribute_def 'Iso8601DateTimeStructure' :> 'Time::UtcTimeInstantValue'[attribute_def]
        (documentation)
        (attribute_usage composite 'year' : 'Integer'[unresolved])
        (attribute_usage composite 'month' : 'Natural'[unresolved])
        (attribute_usage composite 'day' : 'Natural'[unresolved])
        (attribute_usage composite 'hour' : 'Natural'[unresolved])
        (attribute_usage composite 'minute' : 'Natural'[unresolved])
        (attribute_usage composite 'second' : 'Natural'[unresolved])
        (attribute_usage composite 'microsecond' : 'Natural'[unresolved])
        (attribute_usage composite 'hourOffset' : 'Integer'[unresolved])
        (attribute_usage composite 'minuteOffset' : 'Integer'[unresolved])
        (attribute_usage composite :>> 'num'[unresolved]
          (feature_value (=)))
        (calculation_usage composite 'getElapsedUtcTime'
          (reference_usage in reference 'year' : 'Integer'[unresolved])
          (reference_usage in reference 'month' : 'Natural'[unresolved])
          (reference_usage in reference 'day' : 'Natural'[unresolved])
          (reference_usage in reference 'hour' : 'Natural'[unresolved])
          (reference_usage in reference 'minute' : 'Natural'[unresolved])
          (reference_usage in reference 'second' : 'Natural'[unresolved])
          (reference_usage in reference 'microsecond' : 'Natural'[unresolved])
          (reference_usage in reference 'hourOffset' : 'Integer'[unresolved])
          (reference_usage in reference 'minuteOffest' : 'Integer'[unresolved])
          (return_parameter_membership
            (feature_def out : 'Real'[unresolved]))))
      (calculation_usage 'convertIso8601DateTimeToStructure'
        (documentation)
        (reference_usage in reference 'iso8601DateTime' : 'Time::Iso8601DateTime'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'Time::Iso8601DateTimeStructure'[attribute_def])))
      (calculation_usage 'convertIso8601StructureToDateTime'
        (documentation)
        (reference_usage in reference 'iso8601DateTimeStructure' : 'Time::Iso8601DateTimeStructure'[attribute_def])
        (attribute_usage composite 'x' : 'Time::Iso8601DateTime'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'Time::Iso8601DateTime'[attribute_def]))))))
~~~
