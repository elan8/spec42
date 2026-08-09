# META
~~~ini
description=KerML Association: ProductSelection_UnownedEnds
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_UnownedEnds {
	
	class SelectionInfo;
	class ShoppingCart {
		feature selectedProducts : Product[0..*];
	}
	class Product {
		feature inCart: ShoppingCart[0..1];
	}
	
	assoc ProductSelection {
		feature info: SelectionInfo[1];
		
		end feature cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end feature selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	assoc SingleProductSelection :> ProductSelection {
		end feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}
	
	// Equivalent association showing implied relationships explicitly.
	assoc SingleProductSelection1 :> ProductSelection {
		end feature cart: ShoppingCart[1] redefines cart {
			public import selectedProduct::selectedProduct1;
		}
		end feature selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets ShoppingCart::selectedProducts featured by ShoppingCart;
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
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,ColonGt,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwAssoc,Ident,ColonGt,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
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
  (package_def 'ProductSelection_UnownedEnds'
    (class_def 'SelectionInfo')
    (class_def 'ShoppingCart'
      (feature_def 'selectedProducts' : 'Product' multiplicity))
    (class_def 'Product'
      (feature_def 'inCart' : 'ShoppingCart' multiplicity))
    (association_def 'ProductSelection'
      (feature_def 'info' : 'SelectionInfo' multiplicity)
      (feature_def end 'cart' : 'ShoppingCart' multiplicity crosses 'selectedProduct.inCart')
      (feature_def end 'selectedProduct' : 'Product' multiplicity crosses 'cart.selectedProducts'))
    (association_def 'SingleProductSelection' :> 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity))
    (line_comment)
    (association_def 'SingleProductSelection1' :> 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity :>> 'cart'
        (import_decl public 'selectedProduct::selectedProduct1'))
      (feature_def end 'selectedProduct' : 'Product' multiplicity :>> 'selectedProduct' crosses 'cart.selectedProduct1'
        (feature_def member 'selectedProduct1' multiplicity :> 'ShoppingCart::selectedProducts' featured by 'ShoppingCart')))
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
package ProductSelection_UnownedEnds {
    class SelectionInfo;
    class ShoppingCart {
        feature selectedProducts : Product [0..*];
    }
    class Product {
        feature inCart : ShoppingCart [0..1];
    }

    assoc ProductSelection {
        feature info : SelectionInfo [1];

        end feature cart : ShoppingCart [1] crosses selectedProduct.inCart;
        end feature selectedProduct : Product [1] crosses cart.selectedProducts;
    }

    assoc SingleProductSelection :> ProductSelection {
        end feature cart : ShoppingCart [1];
        end feature selectedProduct : Product [1];
    }

    // Equivalent association showing implied relationships explicitly.
    assoc SingleProductSelection1 :> ProductSelection {
        end feature cart : ShoppingCart [1] redefines cart {
            public import selectedProduct::selectedProduct1;
        }
        end feature selectedProduct : Product [1] redefines selectedProduct crosses cart.selectedProduct1 {
            member feature selectedProduct1[0..1] subsets ShoppingCart::selectedProducts featured by ShoppingCart;
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
    (package 'ProductSelection_UnownedEnds'
      (class_def 'SelectionInfo')
      (class_def 'ShoppingCart'
        (feature_def 'selectedProducts' : 'ProductSelection_UnownedEnds::Product'[class_def]
          (multiplicity_range [0..*])))
      (class_def 'Product'
        (feature_def 'inCart' : 'ProductSelection_UnownedEnds::ShoppingCart'[class_def]
          (multiplicity_range [0..1])))
      (association_def 'ProductSelection'
        (feature_def 'info' : 'ProductSelection_UnownedEnds::SelectionInfo'[class_def]
          (multiplicity_range [1]))
        (feature_def end 'cart' : 'ProductSelection_UnownedEnds::ShoppingCart'[class_def] :> 'ProductSelection_UnownedEnds::Product::inCart'[feature_def]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_UnownedEnds::Product'[class_def] :> 'ProductSelection_UnownedEnds::ShoppingCart::selectedProducts'[feature_def]
          (multiplicity_range [1])))
      (association_def 'SingleProductSelection' :> 'ProductSelection_UnownedEnds::ProductSelection'[association_def]
        (feature_def end 'cart' : 'ProductSelection_UnownedEnds::ShoppingCart'[class_def] :>> 'ProductSelection_UnownedEnds::ProductSelection::cart'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_UnownedEnds::Product'[class_def] :>> 'ProductSelection_UnownedEnds::ProductSelection::selectedProduct'[feature_def][implied]
          (multiplicity_range [1])))
      (association_def 'SingleProductSelection1' :> 'ProductSelection_UnownedEnds::ProductSelection'[association_def]
        (feature_def end 'cart' : 'ProductSelection_UnownedEnds::ShoppingCart'[class_def] :>> 'ProductSelection_UnownedEnds::ProductSelection::cart'[feature_def]
          (multiplicity_range [1])
          (membership_import public -> 'ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct::selectedProduct1'[feature_def]))
        (feature_def end 'selectedProduct' : 'ProductSelection_UnownedEnds::Product'[class_def] :>> 'ProductSelection_UnownedEnds::ProductSelection::selectedProduct'[feature_def] :> 'ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct::selectedProduct1'[feature_def]
          (multiplicity_range [1])
          (feature_def 'selectedProduct1' :> 'ProductSelection_UnownedEnds::ShoppingCart::selectedProducts'[feature_def]
            (multiplicity_range [0..1]))))
      (class_def 'OnlineCustomer'
        (feature_def 'info1' : 'ProductSelection_UnownedEnds::SelectionInfo'[class_def])
        (feature_def 'myCart' : 'ProductSelection_UnownedEnds::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (feature_def 'products' : 'ProductSelection_UnownedEnds::Product'[class_def]
          (multiplicity_range [0..*]))
        (connector_def 'ps1' : 'ProductSelection_UnownedEnds::ProductSelection'[association_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (feature_def :>> 'ProductSelection_UnownedEnds::ProductSelection::info'[feature_def]
            (feature_value (=))))
        (connector_def 'ps2' : 'ProductSelection_UnownedEnds::ProductSelection'[association_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (feature_def :>> 'ProductSelection_UnownedEnds::ProductSelection::info'[feature_def]
            (feature_value (=))))))))
~~~
