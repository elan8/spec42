# META
~~~ini
description=KerML Association: ProductSelection_OwnedEnds
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_OwnedEnds {
	
	class SelectionInfo;
	class ShoppingCart;
	class Product;
	
	// User-specified association definition
	assoc ProductSelection {
		feature info: SelectionInfo;
		
		end [0..1] feature cart: ShoppingCart[1];
		end [0..*] nonunique feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with named end features.
	assoc ProductSelection1 {
		feature info: SelectionInfo;
		
		end inCart[0..1] feature cart: ShoppingCart[1];
		end selectedProducts[0..*] feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with nested cross features.
	assoc ProductSelection2 {
		feature info: SelectionInfo;
		
		end feature cart: ShoppingCart[1] { 
			member feature inCart[0..1]; // owned cross feature
		}
		end feature selectedProduct: Product[1] { 
			member feature selectedProducts[0..*]; // owned cross feature
		}
	}
	
	// Equivalent association definition showing library model specialization 
	// and implied cross subsetting.
	assoc ProductSelection3 specializes Links::BinaryLink {
		feature info: SelectionInfo;
		
		end cart: ShoppingCart[1] redefines source crosses selectedProduct.inCart {
			member feature inCart: ShoppingCart[0..1] featured by Product;
			public import selectedProduct::selectedProducts;
		}
		end selectedProduct: Product[1] redefines target crosses cart.selectedProducts {
			member feature selectedProducts: Product[0..*] featured by ShoppingCart;
			public import cart::inCart;
		}
	}
	
	assoc SingleProductSelection specializes ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}

	assoc SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] feature cart: ShoppingCart[1];
		end selectedProduct1 [0..1] feature selectedProduct: Product[1];
	}
	
	assoc SingleProductSelection2 specializes ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart1[0..1]; // owned crossing feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProduct1[0..1]; // owned crossing feature
		}
	}
	
	assoc SingleProductSelection3 specializes ProductSelection3 {
		end cart: ShoppingCart[1] redefines cart crosses selectedProduct.inCart1 {
			member feature inCart1[0..1] subsets inCart featured by Product;
			public import selectedProduct::selectedProduct1;
		}
		end selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets selectedProducts featured by ShoppingCart;
			public import cart::inCart1;
		}
	}
	
	class OnlineCustomer {
		feature info1: SelectionInfo;	
		feature myCart: ShoppingCart[1];	
		feature products: Product[0..*];
		
		connector ps1 : ProductSelection from myCart to products {
			:>> info = info1;
		}
		
		connector ps2 : ProductSelection from [1] myCart to [1] products {
			:>> info = info1;
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
LineComment,
KwAssoc,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwAssoc,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwAssoc,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
CloseCurly,
LineComment,
LineComment,
KwAssoc,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeatured,KwBy,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwFeatured,KwBy,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwFeatured,KwBy,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,Ident,KwTo,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ProductSelection_OwnedEnds'
    (class_def 'SelectionInfo')
    (class_def 'ShoppingCart')
    (class_def 'Product')
    (line_comment)
    (association_def 'ProductSelection'
      (feature_def 'info' : 'SelectionInfo')
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (malformed)
      (feature_def 'selectedProduct' : 'Product' multiplicity))
    (line_comment)
    (association_def 'ProductSelection1'
      (feature_def 'info' : 'SelectionInfo')
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity))
    (line_comment)
    (association_def 'ProductSelection2'
      (feature_def 'info' : 'SelectionInfo')
      (feature_def end 'cart' : 'ShoppingCart' multiplicity
        (feature_def member 'inCart' multiplicity)
        (line_comment))
      (feature_def end 'selectedProduct' : 'Product' multiplicity
        (feature_def member 'selectedProducts' multiplicity)
        (line_comment)))
    (line_comment)
    (line_comment)
    (association_def 'ProductSelection3' :> 'Links::BinaryLink'
      (feature_def 'info' : 'SelectionInfo')
      (feature_def end 'cart' : 'ShoppingCart' multiplicity :>> 'source' crosses 'selectedProduct.inCart'
        (feature_def member 'inCart' : 'ShoppingCart' multiplicity featured by 'Product')
        (import_decl public 'selectedProduct::selectedProducts'))
      (feature_def end 'selectedProduct' : 'Product' multiplicity :>> 'target' crosses 'cart.selectedProducts'
        (feature_def member 'selectedProducts' : 'Product' multiplicity featured by 'ShoppingCart')
        (import_decl public 'cart::inCart')))
    (association_def 'SingleProductSelection' :> 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity))
    (association_def 'SingleProductSelection1' :> 'ProductSelection1'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity))
    (association_def 'SingleProductSelection2' :> 'ProductSelection2'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity
        (feature_def member 'inCart1' multiplicity)
        (line_comment))
      (feature_def end 'selectedProduct' : 'Product' multiplicity
        (feature_def member 'selectedProduct1' multiplicity)
        (line_comment)))
    (association_def 'SingleProductSelection3' :> 'ProductSelection3'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity :>> 'cart' crosses 'selectedProduct.inCart1'
        (feature_def member 'inCart1' multiplicity :> 'inCart' featured by 'Product')
        (import_decl public 'selectedProduct::selectedProduct1'))
      (feature_def end 'selectedProduct' : 'Product' multiplicity :>> 'selectedProduct' crosses 'cart.selectedProduct1'
        (feature_def member 'selectedProduct1' multiplicity :> 'selectedProducts' featured by 'ShoppingCart')
        (import_decl public 'cart::inCart1')))
    (class_def 'OnlineCustomer'
      (feature_def 'info1' : 'SelectionInfo')
      (feature_def 'myCart' : 'ShoppingCart' multiplicity)
      (feature_def 'products' : 'Product' multiplicity)
      (connector_def 'ps1' : 'ProductSelection'
        (connector_end)
        (connector_end)
        (feature_def :>> 'info' value))
      (connector_def 'ps2' : 'ProductSelection'
        (connector_end)
        (connector_end)
        (feature_def :>> 'info' value)))))
~~~
# FORMAT
~~~sysml
package ProductSelection_OwnedEnds {
    class SelectionInfo;
    class ShoppingCart;
    class Product;

    // User-specified association definition
    assoc ProductSelection {
        feature info : SelectionInfo;

        end feature cart : ShoppingCart [1];
        end [0..*] nonunique
        feature selectedProduct : Product [1];
    }

    // Equivalent association definition with named end features.
    assoc ProductSelection1 {
        feature info : SelectionInfo;

        end inCart [0..1] feature cart : ShoppingCart [1];
        end selectedProducts [0..*] feature selectedProduct : Product [1];
    }

    // Equivalent association definition with nested cross features.
    assoc ProductSelection2 {
        feature info : SelectionInfo;

        end feature cart : ShoppingCart [1] {
            member feature inCart[0..1];
            // owned cross feature
        }
        end feature selectedProduct : Product [1] {
            member feature selectedProducts[0..*];
            // owned cross feature
        }
    }

    // Equivalent association definition showing library model specialization 
    // and implied cross subsetting.
    assoc ProductSelection3 specializes Links::BinaryLink {
        feature info : SelectionInfo;

        end cart: ShoppingCart [1] redefines source crosses selectedProduct.inCart {
            member feature inCart : ShoppingCart [0..1] featured by Product;
            public import selectedProduct::selectedProducts;
        }
        end selectedProduct: Product [1] redefines target crosses cart.selectedProducts {
            member feature selectedProducts : Product [0..*] featured by ShoppingCart;
            public import cart::inCart;
        }
    }

    assoc SingleProductSelection specializes ProductSelection {
        end feature cart : ShoppingCart [1];
        end feature selectedProduct : Product [1];
    }

    assoc SingleProductSelection1 specializes ProductSelection1 {
        end inCart1 [0..1] feature cart : ShoppingCart [1];
        end selectedProduct1 [0..1] feature selectedProduct : Product [1];
    }

    assoc SingleProductSelection2 specializes ProductSelection2 {
        end feature cart : ShoppingCart [1] {
            member feature inCart1[0..1];
            // owned crossing feature
        }
        end feature selectedProduct : Product [1] {
            member feature selectedProduct1[0..1];
            // owned crossing feature
        }
    }

    assoc SingleProductSelection3 specializes ProductSelection3 {
        end cart: ShoppingCart [1] redefines cart crosses selectedProduct.inCart1 {
            member feature inCart1[0..1] subsets inCart featured by Product;
            public import selectedProduct::selectedProduct1;
        }
        end selectedProduct: Product [1] redefines selectedProduct crosses cart.selectedProduct1 {
            member feature selectedProduct1[0..1] subsets selectedProducts featured by ShoppingCart;
            public import cart::inCart1;
        }
    }

    class OnlineCustomer {
        feature info1 : SelectionInfo;
        feature myCart : ShoppingCart [1];
        feature products : Product [0..*];

        connector ps1 : ProductSelection from myCart to products {
           :>> info = info1;
        }

        connector ps2 : ProductSelection from [1] myCart to [1] products {
           :>> info = info1;
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'Links::BinaryLink'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'Links::BinaryLink'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ProductSelection_OwnedEnds'
      (class_def 'SelectionInfo')
      (class_def 'ShoppingCart')
      (class_def 'Product')
      (association_def 'ProductSelection'
        (feature_def 'info' : 'ProductSelection_OwnedEnds::SelectionInfo'[class_def])
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (feature_def 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def]
          (multiplicity_range [1])))
      (association_def 'ProductSelection1'
        (feature_def 'info' : 'ProductSelection_OwnedEnds::SelectionInfo'[class_def])
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def]
          (multiplicity_range [1])))
      (association_def 'ProductSelection2'
        (feature_def 'info' : 'ProductSelection_OwnedEnds::SelectionInfo'[class_def])
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def]
          (multiplicity_range [1])
          (feature_def 'inCart'
            (multiplicity_range [0..1])))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def]
          (multiplicity_range [1])
          (feature_def 'selectedProducts'
            (multiplicity_range [0..*]))))
      (association_def 'ProductSelection3' :> 'Links::BinaryLink'[unresolved]
        (feature_def 'info' : 'ProductSelection_OwnedEnds::SelectionInfo'[class_def])
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def] :>> 'source'[unresolved] :> 'ProductSelection_OwnedEnds::ProductSelection3::cart::inCart'[feature_def]
          (multiplicity_range [1])
          (feature_def 'inCart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def]
            (multiplicity_range [0..1]))
          (membership_import public -> 'ProductSelection_OwnedEnds::ProductSelection3::selectedProduct::selectedProducts'[feature_def]))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def] :>> 'target'[unresolved] :> 'ProductSelection_OwnedEnds::ProductSelection3::selectedProduct::selectedProducts'[feature_def]
          (multiplicity_range [1])
          (feature_def 'selectedProducts' : 'ProductSelection_OwnedEnds::Product'[class_def]
            (multiplicity_range [0..*]))
          (membership_import public -> 'ProductSelection_OwnedEnds::ProductSelection3::cart::inCart'[feature_def])))
      (association_def 'SingleProductSelection' :> 'ProductSelection_OwnedEnds::ProductSelection'[association_def]
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection::cart'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection::selectedProduct'[feature_def][implied]
          (multiplicity_range [1])))
      (association_def 'SingleProductSelection1' :> 'ProductSelection_OwnedEnds::ProductSelection1'[association_def]
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection1::cart'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection1::selectedProduct'[feature_def][implied]
          (multiplicity_range [1])))
      (association_def 'SingleProductSelection2' :> 'ProductSelection_OwnedEnds::ProductSelection2'[association_def]
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection2::cart'[feature_def][implied]
          (multiplicity_range [1])
          (feature_def 'inCart1'
            (multiplicity_range [0..1])))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection2::selectedProduct'[feature_def][implied]
          (multiplicity_range [1])
          (feature_def 'selectedProduct1'
            (multiplicity_range [0..1]))))
      (association_def 'SingleProductSelection3' :> 'ProductSelection_OwnedEnds::ProductSelection3'[association_def]
        (feature_def end 'cart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection3::cart'[feature_def] :> 'ProductSelection_OwnedEnds::SingleProductSelection3::cart::inCart1'[feature_def]
          (multiplicity_range [1])
          (feature_def 'inCart1' :> 'ProductSelection_OwnedEnds::ProductSelection3::cart::inCart'[feature_def]
            (multiplicity_range [0..1]))
          (membership_import public -> 'ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct::selectedProduct1'[feature_def]))
        (feature_def end 'selectedProduct' : 'ProductSelection_OwnedEnds::Product'[class_def] :>> 'ProductSelection_OwnedEnds::ProductSelection3::selectedProduct'[feature_def] :> 'ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct::selectedProduct1'[feature_def]
          (multiplicity_range [1])
          (feature_def 'selectedProduct1' :> 'ProductSelection_OwnedEnds::ProductSelection3::selectedProduct::selectedProducts'[feature_def]
            (multiplicity_range [0..1]))
          (membership_import public -> 'ProductSelection_OwnedEnds::SingleProductSelection3::cart::inCart1'[feature_def])))
      (class_def 'OnlineCustomer'
        (feature_def 'info1' : 'ProductSelection_OwnedEnds::SelectionInfo'[class_def])
        (feature_def 'myCart' : 'ProductSelection_OwnedEnds::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (feature_def 'products' : 'ProductSelection_OwnedEnds::Product'[class_def]
          (multiplicity_range [0..*]))
        (connector_def 'ps1' : 'ProductSelection_OwnedEnds::ProductSelection'[association_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (feature_def :>> 'ProductSelection_OwnedEnds::ProductSelection::info'[feature_def]
            (feature_value (=))))
        (connector_def 'ps2' : 'ProductSelection_OwnedEnds::ProductSelection'[association_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (feature_def :>> 'ProductSelection_OwnedEnds::ProductSelection::info'[feature_def]
            (feature_value (=))))))))
~~~
