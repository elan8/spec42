# META
~~~ini
description=SysML Example (Simple Tests): ItemTest
type=file
~~~
# SOURCE
~~~sysml
package ItemTest {
	
	item f: A;

	public item def A {
		item b: B;
		protected ref part c: C;
	}
	
	abstract item def B {
		public abstract part a: A;
	}
	
	private part def C {
		private in ref y: A, B;
	}
	
	port def P {
		in item a1: A;
		out item a2: A;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwPublic,KwItem,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwProtected,KwRef,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwItem,KwDef,Ident,OpenCurly,
KwPublic,KwAbstract,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPrivate,KwPart,KwDef,Ident,OpenCurly,
KwPrivate,KwIn,KwRef,Ident,Colon,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ItemTest'
    (item_usage 'f' : 'A')
    (item_def public 'A'
      (item_usage 'b' : 'B')
      (part_usage protected ref 'c' : 'C'))
    (item_def abstract 'B'
      (part_usage public abstract 'a' : 'A'))
    (part_def private 'C'
      (ref_usage private in ref 'y' : 'A', 'B'))
    (port_def 'P'
      (item_usage in 'a1' : 'A')
      (item_usage out 'a2' : 'A'))))
~~~
# FORMAT
~~~sysml
package ItemTest {
    item f : A;

    public item def A {
        item b : B;
        protected ref part c : C;
    }

    abstract item def B {
        public abstract part a : A;
    }

    private part def C {
        private in ref y : A, B;
    }

    port def P {
        in item a1 : A;
        out item a2 : A;
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'ItemTest'
      (item_usage 'f' : 'ItemTest::A'[item_def])
      (item_def 'A'
        (item_usage composite 'b' : 'ItemTest::B'[item_def])
        (part_usage reference 'c' : 'ItemTest::C'[part_def]))
      (item_def abstract 'B'
        (part_usage abstract composite 'a' : 'ItemTest::A'[item_def]))
      (part_def 'C'
        (reference_usage in reference 'y' : 'ItemTest::A'[item_def] : 'ItemTest::B'[item_def]))
      (port_def 'P'
        (item_usage in 'a1' : 'ItemTest::A'[item_def])
        (item_usage out 'a2' : 'ItemTest::A'[item_def])))))
~~~
