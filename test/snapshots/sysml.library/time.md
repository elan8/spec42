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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Time"))) (name "Time") (declared-name "Time")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Time::Clock"))) (name "Clock") (declared-name "Clock") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::Clock::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Clock")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (name "currentTime") (declared-name "currentTime") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time::Clock")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (name "Coordinated Universal Time") (declared-name "Coordinated Universal Time") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Coordinated Universal Time")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (name "definitionalEpoch") (declared-name "definitionalEpoch") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Coordinated Universal Time")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (name "unit") (declared-name "unit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Coordinated Universal Time")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::Date"))) (name "Date") (declared-name "Date") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::Date::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Date")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::DateTime"))) (name "DateTime") (declared-name "DateTime") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::DateTime::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::DateTime")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Time::DurationOf"))) (name "DurationOf") (declared-name "DurationOf")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::DurationOf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::DurationOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time::DurationOf::clock"))) (name "clock") (declared-name "clock") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Time::DurationOf")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Time::DurationOf::duration"))) (name "duration") (declared-name "duration") (effective (featuring-type (node (document "d0") (qualified-name "Time::DurationOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time::DurationOf::o"))) (name "o") (declared-name "o") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Time::DurationOf")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::DurationUnit"))) (name "DurationUnit") (declared-name "DurationUnit"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::DurationValue"))) (name "DurationValue") (declared-name "DurationValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::Integer"))) (name "Integer") (declared-name "Integer"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (name "Iso8601DateTime") (declared-name "Iso8601DateTime") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::Iso8601DateTime::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTime")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTime")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (name "val") (declared-name "val") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTime")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (name "Iso8601DateTimeEncoding") (declared-name "Iso8601DateTimeEncoding") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (name "Iso8601DateTimeStructure") (declared-name "Iso8601DateTimeStructure") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::day"))) (name "day") (declared-name "day") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hour"))) (name "hour") (declared-name "hour") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hourOffset"))) (name "hourOffset") (declared-name "hourOffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::microsecond"))) (name "microsecond") (declared-name "microsecond") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minute"))) (name "minute") (declared-name "minute") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minuteOffset"))) (name "minuteOffset") (declared-name "minuteOffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::month"))) (name "month") (declared-name "month") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::second"))) (name "second") (declared-name "second") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::year"))) (name "year") (declared-name "year") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::String"))) (name "String") (declared-name "String"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (name "TimeInstantValue") (declared-name "TimeInstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::TimeInstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Time::TimeOf"))) (name "TimeOf") (declared-name "TimeOf") (declared (own-expression (expression (kind "featureReference") (reference "timeInstant")))) (evaluation (expression (status "incomplete") (error "expression is incomplete")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::TimeOf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::TimeOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time::TimeOf::clock"))) (name "clock") (declared-name "clock") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Time::TimeOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time::TimeOf::o"))) (name "o") (declared-name "o") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Time::TimeOf")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::TimeOfDay"))) (name "TimeOfDay") (declared-name "TimeOfDay") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::TimeOfDay::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::TimeOfDay")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::TimeScale"))) (name "TimeScale") (declared-name "TimeScale") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::TimeScale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::TimeScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::TimeScale::definitionalEpoch"))) (name "definitionalEpoch") (declared-name "definitionalEpoch") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::TimeScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::TimeScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (name "unit") (declared-name "unit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::TimeScale")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::TimeUnit"))) (name "TimeUnit") (declared-name "TimeUnit"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::TimeValue"))) (name "TimeValue") (declared-name "TimeValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (name "UtcTimeInstantValue") (declared-name "UtcTimeInstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::UtcTimeInstantValue")))))
              )
            )
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::_documentation"))) (name ""))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))) (name "convertIso8601DateTimeToStructure") (declared-name "convertIso8601DateTimeToStructure")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::iso8601DateTime"))) (name "iso8601DateTime") (declared-name "iso8601DateTime") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))) (name "convertIso8601StructureToDateTime") (declared-name "convertIso8601StructureToDateTime") (declared (own-expression (expression (kind "featureReference") (reference "attribute")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::iso8601DateTimeStructure"))) (name "iso8601DateTimeStructure") (declared-name "iso8601DateTimeStructure") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::duration"))) (name "duration") (declared-name "duration"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::scalarQuantities"))) (name "scalarQuantities") (declared-name "scalarQuantities"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time::time"))) (name "time") (declared-name "time"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::timeInstant"))) (name "timeInstant") (declared-name "timeInstant") (declared (properties (ordered false) (unique true))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Time::universalClock"))) (name "universalClock") (declared-name "universalClock") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Time::universalClock::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Time::Clock")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (name "utcTimeInstant") (declared-name "utcTimeInstant") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::Clock::_documentation"))) (to (node (document "d0") (qualified-name "Time::Clock"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time::_documentation"))) (to (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::Date::_documentation"))) (to (node (document "d0") (qualified-name "Time::Date"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::DateTime::_documentation"))) (to (node (document "d0") (qualified-name "Time::DateTime"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::DurationOf::_documentation"))) (to (node (document "d0") (qualified-name "Time::DurationOf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTime::_documentation"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding::_documentation"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::_documentation"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::TimeInstantValue::_documentation"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::TimeOf::_documentation"))) (to (node (document "d0") (qualified-name "Time::TimeOf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::TimeOfDay::_documentation"))) (to (node (document "d0") (qualified-name "Time::TimeOfDay"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::TimeScale::_documentation"))) (to (node (document "d0") (qualified-name "Time::TimeScale"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef::_documentation"))) (to (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::_documentation"))) (to (node (document "d0") (qualified-name "Time"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::_documentation"))) (to (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::_documentation"))) (to (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Time::universalClock::_documentation"))) (to (node (document "d0") (qualified-name "Time::universalClock"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (to (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (to (node (document "d0") (qualified-name "Time::TimeScale::definitionalEpoch"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (to (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Time::Clock"))) (to (node (document "d0") (qualified-name "Time::Clock"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (to (node (document "d0") (qualified-name "Time::TimeScale"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::Date"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::DateTime"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (to (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (to (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (to (node (document "d0") (qualified-name "Time::TimeScale"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::TimeOfDay"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (to (node (document "d0") (qualified-name "Time::DateTime"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::iso8601DateTime"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::iso8601DateTimeStructure"))) (to (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::timeInstant"))) (to (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::universalClock"))) (to (node (document "d0") (qualified-name "Time::Clock"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (to (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Clock"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Date"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::DateTime"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::DurationOf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::day"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hour"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hourOffset"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::microsecond"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minute"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minuteOffset"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::month"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::second"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::year"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeOf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeOfDay"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeScale"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeScale::definitionalEpoch"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::timeInstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::universalClock"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/time.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 18) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 18) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 18) (end 18 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 18) (end 19 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 18) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 18) (end 21 36))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 30 1) (end 30 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 2) (end 47 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 2) (end 48 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 2) (end 59 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 2) (end 60 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 2) (end 61 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 4) (end 64 499))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 72 2) (end 72 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 2) (end 72 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 2) (end 73 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 74 2) (end 74 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 4) (end 77 271))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 85 8) (end 85 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 8) (end 85 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 86 8) (end 86 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 132 2) (end 132 153))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 225 5) (end 225 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 226 5) (end 226 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 227 5) (end 227 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 5) (end 228 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 229 5) (end 229 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 230 5) (end 230 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 231 5) (end 231 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 232 5) (end 232 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 233 5) (end 233 37))
      )
    )
  )
)
~~~
