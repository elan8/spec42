// A specialization chain with a shadowed feature name, pinning the effective-features
// ordering rule: direct before inherited, nearest-first, nearer shadows farther.
package Inheritance {
    part def Root {
        attribute shared;
        attribute onlyRoot;
    }

    part def Middle :> Root {
        attribute shared;
        attribute onlyMiddle;
    }

    part def Leaf :> Middle {
        attribute shared;
        attribute onlyLeaf;
    }
}
