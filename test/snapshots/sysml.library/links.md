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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "links.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 31))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "eba81b58d46e6326f4a7605d66c21a1206821ebd35a1d9c0cb11cafc7643bd28") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Links"))) (kind "package") (name "Links") (declared-name "Links") (range (start (line 0) (character 0)) (end (line 0) (character 2117))))
    (element (id (node (document "d0") (qualified-name "Links::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 6) (character 4)) (end (line 6) (character 34))) (parent (node (document "d0") (qualified-name "Links"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 33))))))
    (element (id (node (document "d0") (qualified-name "Links::Link"))) (kind "kermlDecl") (name "Link") (declared-name "Link") (range (start (line 9) (character 4)) (end (line 9) (character 227))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2117))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::all"))) (kind "kermlDecl") (name "all") (declared-name "all") (range (start (line 18) (character 4)) (end (line 18) (character 456))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::all#kermlDecl"))) (kind "kermlDecl") (name "all") (declared-name "all") (range (start (line 31) (character 4)) (end (line 31) (character 402))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::binaryLinks"))) (kind "feature decl") (name "binaryLinks") (declared-name "binaryLinks") (range (start (line 49) (character 4)) (end (line 49) (character 200))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::links"))) (kind "feature decl") (name "links") (declared-name "links") (range (start (line 42) (character 4)) (end (line 42) (character 181))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::selfLinks"))) (kind "feature decl") (name "selfLinks") (declared-name "selfLinks") (range (start (line 56) (character 4)) (end (line 56) (character 389))) (parent (node (document "d0") (qualified-name "Links"))))
    (element (id (node (document "d0") (qualified-name "Links::things"))) (kind "import") (name "things") (declared-name "things") (range (start (line 7) (character 4)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "Links"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::things") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 31))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Links::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 6) (character 19)) (end (line 6) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Links::things"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::things") (range (start (line 7) (character 19)) (end (line 7) (character 31))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 19) (end 7 31)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Links::things"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::things")
        (range (start 7 19) (end 7 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 19) (end 6 33)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "Links::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 19) (end 6 33))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
