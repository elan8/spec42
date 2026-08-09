# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Links
type=file
~~~
# SOURCE
~~~kerml
standard library package Links {
    doc
    /*
     * This package defines associations and features that are related to the typing of links.
     */

    private import Base::Anything;
    private import Base::things;
    
    abstract assoc Link specializes Anything {
        doc
        /*
         * Link is the most general association between two or more things.
         */

        feature participant: Anything[2..*] nonunique ordered;
    }
    
    assoc all BinaryLink specializes Link {
        doc
        /*
         * BinaryLink is the most general binary association between exactly two things, 
         * nominally directed from source to target.
         */
         
        feature participant: Anything[2] nonunique ordered redefines Link::participant;
        
        end feature source: Anything[1] subsets participant;
        end feature target: Anything[1] subsets participant;
    }
    
    assoc all SelfLink specializes BinaryLink {
        doc
        /*
         * SelfLink is a binary association in which the things at the two ends are asserted
         * to be the same.
         */
        
        end feature thisThing: Anything redefines source subsets sameThing crosses sameThing.self;
        end self2 [1] feature sameThing: Anything redefines target subsets thisThing;
    }
        
    abstract feature links: Link[0..*] nonunique subsets things {
        doc
        /*
         * links is the most general feature of links between individuals.
         */
    }
    
    abstract feature binaryLinks: BinaryLink[0..*] nonunique subsets links {
        doc
        /*
         * binaryLinks is a specialization of links restricted to type BinaryLink.
         */
    }
    
    abstract feature selfLinks: SelfLink[0..*] nonunique subsets binaryLinks {
        doc
        /*
         * selfLinks is a specialization of binaryLinks restricted to type SelfLink.
         */

        end feature thisThing: Anything redefines SelfLink::thisThing, binaryLinks::source;
        end feature sameThing: Anything redefines SelfLink::sameThing, binaryLinks::target;
    }

}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'sameThing::self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'things'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'sameThing::self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'things'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwOrdered,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwNonunique,KwOrdered,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwSubsets,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Links'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Base::things')
    (association_def abstract 'Link' :> 'Anything'
      (documentation)
      (feature_def 'participant' : 'Anything' multiplicity ordered nonunique))
    (association_def all 'BinaryLink' :> 'Link'
      (documentation)
      (feature_def 'participant' : 'Anything' multiplicity :>> 'Link::participant' ordered nonunique)
      (feature_def end 'source' : 'Anything' multiplicity :> 'participant')
      (feature_def end 'target' : 'Anything' multiplicity :> 'participant'))
    (association_def all 'SelfLink' :> 'BinaryLink'
      (documentation)
      (feature_def end 'thisThing' : 'Anything' :>> 'source' :> 'sameThing' crosses 'sameThing.self')
      (feature_def end 'sameThing' multiplicity : 'Anything' :>> 'target' :> 'thisThing'))
    (feature_def abstract 'links' : 'Link' multiplicity :> 'things' nonunique
      (documentation))
    (feature_def abstract 'binaryLinks' : 'BinaryLink' multiplicity :> 'links' nonunique
      (documentation))
    (feature_def abstract 'selfLinks' : 'SelfLink' multiplicity :> 'binaryLinks' nonunique
      (documentation)
      (feature_def end 'thisThing' : 'Anything' :>> 'SelfLink::thisThing', 'binaryLinks::source')
      (feature_def end 'sameThing' : 'Anything' :>> 'SelfLink::sameThing', 'binaryLinks::target'))))
~~~
# FORMAT
~~~sysml
standard library package Links {
    doc /*
     * This package defines associations and features that are related to the typing of links.
     */

    private import Base::Anything;
    private import Base::things;

    abstract assoc Link specializes Anything {
        doc /*
         * Link is the most general association between two or more things.
         */

        feature participant : Anything [2..*] ordered nonunique;
    }

    assoc all BinaryLink specializes Link {
        doc /*
         * BinaryLink is the most general binary association between exactly two things, 
         * nominally directed from source to target.
         */

        feature participant : Anything [2] redefines Link::participant ordered nonunique;

        end feature source : Anything [1] subsets participant;
        end feature target : Anything [1] subsets participant;
    }

    assoc all SelfLink specializes BinaryLink {
        doc /*
         * SelfLink is a binary association in which the things at the two ends are asserted
         * to be the same.
         */

        end feature thisThing : Anything redefines source subsets sameThing crosses sameThing.self;
        end self2 [1] feature sameThing : Anything redefines target subsets thisThing;
    }

    abstract feature links : Link [0..*] subsets things nonunique {
        doc /*
         * links is the most general feature of links between individuals.
         */
    }

    abstract feature binaryLinks : BinaryLink [0..*] subsets links nonunique {
        doc /*
         * binaryLinks is a specialization of links restricted to type BinaryLink.
         */
    }

    abstract feature selfLinks : SelfLink [0..*] subsets binaryLinks nonunique {
        doc /*
         * selfLinks is a specialization of binaryLinks restricted to type SelfLink.
         */

        end feature thisThing : Anything redefines SelfLink::thisThing, binaryLinks::source;
        end feature sameThing : Anything redefines SelfLink::sameThing, binaryLinks::target;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Links'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Base::things'[unresolved])
      (association_def abstract 'Link' :> 'Anything'[unresolved]
        (documentation)
        (feature_def ordered 'participant' : 'Anything'[unresolved]
          (multiplicity_range [2..*])))
      (association_def sufficient 'BinaryLink' :> 'Links::Link'[association_def]
        (documentation)
        (feature_def ordered 'participant' : 'Anything'[unresolved] :>> 'Links::Link::participant'[feature_def]
          (multiplicity_range [2]))
        (feature_def end 'source' : 'Anything'[unresolved] :> 'Links::BinaryLink::participant'[feature_def]
          (multiplicity_range [1]))
        (feature_def end 'target' : 'Anything'[unresolved] :> 'Links::BinaryLink::participant'[feature_def]
          (multiplicity_range [1])))
      (association_def sufficient 'SelfLink' :> 'Links::BinaryLink'[association_def]
        (documentation)
        (feature_def end 'thisThing' : 'Anything'[unresolved] :>> 'Links::BinaryLink::source'[feature_def] :> 'Links::SelfLink::sameThing'[feature_def] :> 'sameThing::self'[unresolved])
        (feature_def end 'sameThing' : 'Anything'[unresolved] :>> 'Links::BinaryLink::target'[feature_def] :> 'Links::SelfLink::thisThing'[feature_def]
          (multiplicity_range [1])))
      (feature_def abstract 'links' : 'Links::Link'[association_def] :> 'things'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (feature_def abstract 'binaryLinks' : 'Links::BinaryLink'[association_def] :> 'Links::links'[feature_def]
        (multiplicity_range [0..*])
        (documentation))
      (feature_def abstract 'selfLinks' : 'Links::SelfLink'[association_def] :> 'Links::binaryLinks'[feature_def]
        (multiplicity_range [0..*])
        (documentation)
        (feature_def end 'thisThing' : 'Anything'[unresolved] :>> 'Links::SelfLink::thisThing'[feature_def] :>> 'Links::BinaryLink::source'[feature_def])
        (feature_def end 'sameThing' : 'Anything'[unresolved] :>> 'Links::SelfLink::sameThing'[feature_def] :>> 'Links::BinaryLink::target'[feature_def])))))
~~~
