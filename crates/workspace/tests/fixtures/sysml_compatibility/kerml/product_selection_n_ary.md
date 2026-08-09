# META
~~~ini
description=KerML Association: ProductSelection_N_ary
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_N_ary {
	
	class ShoppingCart;
	class Product;
	class Account;
	
	// User-specified association definition
	assoc ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..*] feature selectedProduct: Product[1];
		end [1..1] feature account : Account[1];
	}
	
	// Equivalent association definition with named end features.
	assoc ProductSelection1 {
		end inCart[0..1] feature cart: ShoppingCart[1];
		end selectedProducts[0..*] feature selectedProduct: Product[1];
		end withAccount[1..1] feature account : Account[1];
	}
	
	// Equivalent association definition with nested cross features.
	assoc ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart[0..1]; // owned cross feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProducts[0..*]; // owned cross feature
		}
		end feature account : Account[1] {
			member feature withAccount[1..1]; // owned cross feature
		}
	}
	
	// Equivalent association definition showing library model specialization 
	// implied cross subsetting, and "Cartesian product" features.
	assoc ProductSelection3 specializes Links::Link {
		end cart: ShoppingCart[1] crosses cart::product_account.inCart {
			member feature inCart: ShoppingCart[0..1] featured by Product_Account {
                // Represents the "Cartesian product" of Product X Account.
				member feature Product_Account : Account featured by Product;
			}
			member feature product_account : inCart::Product_Account featured by ProductSelection3 {
				public import inCart;
			}
		}
		end selectedProduct: Product[1] crosses selectedProduct::cart_account.selectedProducts {
			member feature selectedProducts: Product[0..*] featured by Cart_Account {
                // Represents the "Cartesian product" of ShoppingCart X Account.
				member feature Cart_Account : Account featured by ShoppingCart;				
			}
			member feature cart_account : selectedProducts::Cart_Account featured by ProductSelection3 {
				public import selectedProducts;
			}
		}
		end feature account : Account[1] crosses account::cart_product.withAccount {
			member feature withAccount[1..1] : Account featured by Cart_Product {
                // Represents the "Cartesian product" of ShoppingCart X Product.
				member feature Cart_Product : Product featured by ShoppingCart;
			}
			member feature cart_product : withAccount::Cart_Product featured by ProductSelection3 {
				public import withAccount;
			}
		}
	}
	
	assoc SingleProductSelection specializes ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
		end [1..1] feature account : Account[1];
	}

	assoc SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] feature cart: ShoppingCart[1];
		end selectedProduct1 [0..1] feature selectedProduct: Product[1];
		end withAccount1 [1..1] feature account : Account[1];
	}
	
	assoc SingleProductSelection2 specializes ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart1[0..1]; // owned crossing feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProducts1[0..*]; // owned crossing feature
		}
		end feature account : Account[1] {
			member feature withAccount1[0..*]; // owned crossing feature
		}
	}
	
	assoc SingleProductSelection3 specializes ProductSelection3 {
		end cart: ShoppingCart[1] redefines cart crosses cart::product_account1.inCart1 {
			member feature inCart1: ShoppingCart[0..1] featured by Product_Account1 {
				member feature Product_Account1 subsets Product_Account : Account featured by Product;
			}
			member feature product_account1 : inCart1::Product_Account1 featured by ProductSelection3 {
				public import inCart1;
			}
		}
		end selectedProduct: Product[1] redefines selectedProduct crosses selectedProduct::cart_account1.selectedProduct1 {
			member feature selectedProduct1: Product[1..1] featured by Cart_Account1 {
				member feature Cart_Account1 subsets Cart_Account : Account featured by ShoppingCart;				
			}
			member feature cart_account1 : selectedProduct1::Cart_Account1 featured by ProductSelection3 {
				public import selectedProduct1;
			}
		}
		end feature account : Account[1] crosses account::cart_product1.withAccount1 {
			member feature withAccount1[1..1] : Account featured by cart_product1 {
				member feature Cart_Product1 subsets Cart_Product : Product featured by ShoppingCart;
			}
			member feature cart_product1 : withAccount1::Cart_Product1 featured by ProductSelection3 {
				public import withAccount1;
			}
		}
	}
	
	class OnlineCustomer {
		feature myCart: ShoppingCart[1];	
		feature products: Product[0..*];
		feature myAccount : Account[1];
		
		connector ps1 : ProductSelection (myCart, products, myAccount);
		
		connector ps2 : ProductSelection ([1] myCart, [0..1] products, [1] myAccount);
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
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwAssoc,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwAssoc,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
CloseCurly,
LineComment,
LineComment,
KwAssoc,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,ColonColon,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,Colon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,ColonColon,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,Colon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,ColonColon,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,Colon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,ColonColon,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,KwSubsets,Ident,Colon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwEnd,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,ColonColon,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,KwSubsets,Ident,Colon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,ColonColon,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,KwSubsets,Ident,Colon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwConnector,Ident,Colon,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwConnector,Ident,Colon,Ident,OpenParen,OpenSquare,DecimalValue,CloseSquare,Ident,Comma,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Comma,OpenSquare,DecimalValue,CloseSquare,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ProductSelection_N_ary'
    (class_def 'ShoppingCart')
    (class_def 'Product')
    (class_def 'Account')
    (line_comment)
    (association_def 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity)
      (feature_def end 'account' : 'Account' multiplicity))
    (line_comment)
    (association_def 'ProductSelection1'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity)
      (feature_def end 'account' : 'Account' multiplicity))
    (line_comment)
    (association_def 'ProductSelection2'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity
        (feature_def member 'inCart' multiplicity)
        (line_comment))
      (feature_def end 'selectedProduct' : 'Product' multiplicity
        (feature_def member 'selectedProducts' multiplicity)
        (line_comment))
      (feature_def end 'account' : 'Account' multiplicity
        (feature_def member 'withAccount' multiplicity)
        (line_comment)))
    (line_comment)
    (line_comment)
    (association_def 'ProductSelection3' :> 'Links::Link'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity crosses 'cart::product_account.inCart'
        (feature_def member 'inCart' : 'ShoppingCart' multiplicity featured by 'Product_Account'
          (line_comment)
          (feature_def member 'Product_Account' : 'Account' featured by 'Product'))
        (feature_def member 'product_account' : 'inCart::Product_Account' featured by 'ProductSelection3'
          (import_decl public 'inCart')))
      (feature_def end 'selectedProduct' : 'Product' multiplicity crosses 'selectedProduct::cart_account.selectedProducts'
        (feature_def member 'selectedProducts' : 'Product' multiplicity featured by 'Cart_Account'
          (line_comment)
          (feature_def member 'Cart_Account' : 'Account' featured by 'ShoppingCart'))
        (feature_def member 'cart_account' : 'selectedProducts::Cart_Account' featured by 'ProductSelection3'
          (import_decl public 'selectedProducts')))
      (feature_def end 'account' : 'Account' multiplicity crosses 'account::cart_product.withAccount'
        (feature_def member 'withAccount' multiplicity : 'Account' featured by 'Cart_Product'
          (line_comment)
          (feature_def member 'Cart_Product' : 'Product' featured by 'ShoppingCart'))
        (feature_def member 'cart_product' : 'withAccount::Cart_Product' featured by 'ProductSelection3'
          (import_decl public 'withAccount'))))
    (association_def 'SingleProductSelection' :> 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity)
      (feature_def end 'account' : 'Account' multiplicity))
    (association_def 'SingleProductSelection1' :> 'ProductSelection1'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity)
      (feature_def end 'account' : 'Account' multiplicity))
    (association_def 'SingleProductSelection2' :> 'ProductSelection2'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity
        (feature_def member 'inCart1' multiplicity)
        (line_comment))
      (feature_def end 'selectedProduct' : 'Product' multiplicity
        (feature_def member 'selectedProducts1' multiplicity)
        (line_comment))
      (feature_def end 'account' : 'Account' multiplicity
        (feature_def member 'withAccount1' multiplicity)
        (line_comment)))
    (association_def 'SingleProductSelection3' :> 'ProductSelection3'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity :>> 'cart' crosses 'cart::product_account1.inCart1'
        (feature_def member 'inCart1' : 'ShoppingCart' multiplicity featured by 'Product_Account1'
          (feature_def member 'Product_Account1' :> 'Product_Account' : 'Account' featured by 'Product'))
        (feature_def member 'product_account1' : 'inCart1::Product_Account1' featured by 'ProductSelection3'
          (import_decl public 'inCart1')))
      (feature_def end 'selectedProduct' : 'Product' multiplicity :>> 'selectedProduct' crosses 'selectedProduct::cart_account1.selectedProduct1'
        (feature_def member 'selectedProduct1' : 'Product' multiplicity featured by 'Cart_Account1'
          (feature_def member 'Cart_Account1' :> 'Cart_Account' : 'Account' featured by 'ShoppingCart'))
        (feature_def member 'cart_account1' : 'selectedProduct1::Cart_Account1' featured by 'ProductSelection3'
          (import_decl public 'selectedProduct1')))
      (feature_def end 'account' : 'Account' multiplicity crosses 'account::cart_product1.withAccount1'
        (feature_def member 'withAccount1' multiplicity : 'Account' featured by 'cart_product1'
          (feature_def member 'Cart_Product1' :> 'Cart_Product' : 'Product' featured by 'ShoppingCart'))
        (feature_def member 'cart_product1' : 'withAccount1::Cart_Product1' featured by 'ProductSelection3'
          (import_decl public 'withAccount1'))))
    (class_def 'OnlineCustomer'
      (feature_def 'myCart' : 'ShoppingCart' multiplicity)
      (feature_def 'products' : 'Product' multiplicity)
      (feature_def 'myAccount' : 'Account' multiplicity)
      (malformed)
      (malformed))))
~~~
# FORMAT
~~~sysml
package ProductSelection_N_ary {
    class ShoppingCart;
    class Product;
    class Account;

    // User-specified association definition
    assoc ProductSelection {
        end feature cart : ShoppingCart [1];
        end feature selectedProduct : Product [1];
        end feature account : Account [1];
    }

    // Equivalent association definition with named end features.
    assoc ProductSelection1 {
        end inCart [0..1] feature cart : ShoppingCart [1];
        end selectedProducts [0..*] feature selectedProduct : Product [1];
        end withAccount [1..1] feature account : Account [1];
    }

    // Equivalent association definition with nested cross features.
    assoc ProductSelection2 {
        end feature cart : ShoppingCart [1] {
            member feature inCart[0..1];
            // owned cross feature
        }
        end feature selectedProduct : Product [1] {
            member feature selectedProducts[0..*];
            // owned cross feature
        }
        end feature account : Account [1] {
            member feature withAccount[1..1];
            // owned cross feature
        }
    }

    // Equivalent association definition showing library model specialization 
    // implied cross subsetting, and "Cartesian product" features.
    assoc ProductSelection3 specializes Links::Link {
        end cart: ShoppingCart [1] crosses cart::product_account.inCart {
            member feature inCart : ShoppingCart [0..1] featured by Product_Account {
                // Represents the "Cartesian product" of Product X Account.
                member feature Product_Account : Account featured by Product;
            }
            member feature product_account : inCart::Product_Account featured by ProductSelection3 {
                public import inCart;
            }
        }
        end selectedProduct: Product [1] crosses selectedProduct::cart_account.selectedProducts {
            member feature selectedProducts : Product [0..*] featured by Cart_Account {
                // Represents the "Cartesian product" of ShoppingCart X Account.
                member feature Cart_Account : Account featured by ShoppingCart;
            }
            member feature cart_account : selectedProducts::Cart_Account featured by ProductSelection3 {
                public import selectedProducts;
            }
        }
        end feature account : Account [1] crosses account::cart_product.withAccount {
            member feature withAccount[1..1] : Account featured by Cart_Product {
                // Represents the "Cartesian product" of ShoppingCart X Product.
                member feature Cart_Product : Product featured by ShoppingCart;
            }
            member feature cart_product : withAccount::Cart_Product featured by ProductSelection3 {
                public import withAccount;
            }
        }
    }

    assoc SingleProductSelection specializes ProductSelection {
        end feature cart : ShoppingCart [1];
        end feature selectedProduct : Product [1];
        end feature account : Account [1];
    }

    assoc SingleProductSelection1 specializes ProductSelection1 {
        end inCart1 [0..1] feature cart : ShoppingCart [1];
        end selectedProduct1 [0..1] feature selectedProduct : Product [1];
        end withAccount1 [1..1] feature account : Account [1];
    }

    assoc SingleProductSelection2 specializes ProductSelection2 {
        end feature cart : ShoppingCart [1] {
            member feature inCart1[0..1];
            // owned crossing feature
        }
        end feature selectedProduct : Product [1] {
            member feature selectedProducts1[0..*];
            // owned crossing feature
        }
        end feature account : Account [1] {
            member feature withAccount1[0..*];
            // owned crossing feature
        }
    }

    assoc SingleProductSelection3 specializes ProductSelection3 {
        end cart: ShoppingCart [1] redefines cart crosses cart::product_account1.inCart1 {
            member feature inCart1 : ShoppingCart [0..1] featured by Product_Account1 {
                member feature Product_Account1 subsets Product_Account : Account featured by Product;
            }
            member feature product_account1 : inCart1::Product_Account1 featured by ProductSelection3 {
                public import inCart1;
            }
        }
        end selectedProduct: Product [1] redefines selectedProduct crosses selectedProduct::cart_account1.selectedProduct1 {
            member feature selectedProduct1 : Product [1..1] featured by Cart_Account1 {
                member feature Cart_Account1 subsets Cart_Account : Account featured by ShoppingCart;
            }
            member feature cart_account1 : selectedProduct1::Cart_Account1 featured by ProductSelection3 {
                public import selectedProduct1;
            }
        }
        end feature account : Account [1] crosses account::cart_product1.withAccount1 {
            member feature withAccount1[1..1] : Account featured by cart_product1 {
                member feature Cart_Product1 subsets Cart_Product : Product featured by ShoppingCart;
            }
            member feature cart_product1 : withAccount1::Cart_Product1 featured by ProductSelection3 {
                public import withAccount1;
            }
        }
    }

    class OnlineCustomer {
        feature myCart : ShoppingCart [1];
        feature products : Product [0..*];
        feature myAccount : Account [1];

        connector ps1 : ProductSelection (myCart, products, myAccount);

        connector ps2 : ProductSelection ([1] myCart, [0..1] products, [1] myAccount);
    }
}
~~~
# EXPECTED
~~~
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'Links::Link'
semantic.unresolved_name 'cart::product_account::inCart'
semantic.unresolved_name 'Product_Account'
semantic.unresolved_name 'selectedProduct::cart_account::selectedProducts'
semantic.unresolved_name 'Cart_Account'
semantic.unresolved_name 'account::cart_product::withAccount'
semantic.unresolved_name 'Cart_Product'
semantic.unresolved_name 'cart::product_account1::inCart1'
semantic.unresolved_name 'Product_Account1'
semantic.unresolved_name 'Product_Account'
semantic.unresolved_name 'selectedProduct::cart_account1::selectedProduct1'
semantic.unresolved_name 'Cart_Account1'
semantic.unresolved_name 'Cart_Account'
semantic.unresolved_name 'account::cart_product1::withAccount1'
semantic.unresolved_name 'Cart_Product'
~~~
# PROBLEMS
~~~
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'Links::Link'
semantic.unresolved_name 'cart::product_account::inCart'
semantic.unresolved_name 'Product_Account'
semantic.unresolved_name 'selectedProduct::cart_account::selectedProducts'
semantic.unresolved_name 'Cart_Account'
semantic.unresolved_name 'account::cart_product::withAccount'
semantic.unresolved_name 'Cart_Product'
semantic.unresolved_name 'cart::product_account1::inCart1'
semantic.unresolved_name 'Product_Account1'
semantic.unresolved_name 'Product_Account'
semantic.unresolved_name 'selectedProduct::cart_account1::selectedProduct1'
semantic.unresolved_name 'Cart_Account1'
semantic.unresolved_name 'Cart_Account'
semantic.unresolved_name 'account::cart_product1::withAccount1'
semantic.unresolved_name 'Cart_Product'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ProductSelection_N_ary'
      (class_def 'ShoppingCart')
      (class_def 'Product')
      (class_def 'Account')
      (association_def 'ProductSelection'
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def]
          (multiplicity_range [1]))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def]
          (multiplicity_range [1])))
      (association_def 'ProductSelection1'
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def]
          (multiplicity_range [1]))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def]
          (multiplicity_range [1])))
      (association_def 'ProductSelection2'
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def]
          (multiplicity_range [1])
          (feature_def 'inCart'
            (multiplicity_range [0..1])))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def]
          (multiplicity_range [1])
          (feature_def 'selectedProducts'
            (multiplicity_range [0..*])))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def]
          (multiplicity_range [1])
          (feature_def 'withAccount'
            (multiplicity_range [1..1]))))
      (association_def 'ProductSelection3' :> 'Links::Link'[unresolved]
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def] :> 'cart::product_account::inCart'[unresolved]
          (multiplicity_range [1])
          (feature_def 'inCart' : 'ProductSelection_N_ary::ShoppingCart'[class_def]
            (multiplicity_range [0..1])
            (feature_def 'Product_Account' : 'ProductSelection_N_ary::Account'[class_def]))
          (feature_def 'product_account' : 'ProductSelection_N_ary::ProductSelection3::cart::inCart::Product_Account'[feature_def]
            (membership_import public -> 'ProductSelection_N_ary::ProductSelection3::cart::inCart'[feature_def])))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def] :> 'selectedProduct::cart_account::selectedProducts'[unresolved]
          (multiplicity_range [1])
          (feature_def 'selectedProducts' : 'ProductSelection_N_ary::Product'[class_def]
            (multiplicity_range [0..*])
            (feature_def 'Cart_Account' : 'ProductSelection_N_ary::Account'[class_def]))
          (feature_def 'cart_account' : 'ProductSelection_N_ary::ProductSelection3::selectedProduct::selectedProducts::Cart_Account'[feature_def]
            (membership_import public -> 'ProductSelection_N_ary::ProductSelection3::selectedProduct::selectedProducts'[feature_def])))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def] :> 'account::cart_product::withAccount'[unresolved]
          (multiplicity_range [1])
          (feature_def 'withAccount' : 'ProductSelection_N_ary::Account'[class_def]
            (multiplicity_range [1..1])
            (feature_def 'Cart_Product' : 'ProductSelection_N_ary::Product'[class_def]))
          (feature_def 'cart_product' : 'ProductSelection_N_ary::ProductSelection3::account::withAccount::Cart_Product'[feature_def]
            (membership_import public -> 'ProductSelection_N_ary::ProductSelection3::account::withAccount'[feature_def]))))
      (association_def 'SingleProductSelection' :> 'ProductSelection_N_ary::ProductSelection'[association_def]
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def] :>> 'ProductSelection_N_ary::ProductSelection::cart'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def] :>> 'ProductSelection_N_ary::ProductSelection::selectedProduct'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def] :>> 'ProductSelection_N_ary::ProductSelection::account'[feature_def][implied]
          (multiplicity_range [1])))
      (association_def 'SingleProductSelection1' :> 'ProductSelection_N_ary::ProductSelection1'[association_def]
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def] :>> 'ProductSelection_N_ary::ProductSelection1::cart'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def] :>> 'ProductSelection_N_ary::ProductSelection1::selectedProduct'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def] :>> 'ProductSelection_N_ary::ProductSelection1::account'[feature_def][implied]
          (multiplicity_range [1])))
      (association_def 'SingleProductSelection2' :> 'ProductSelection_N_ary::ProductSelection2'[association_def]
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def] :>> 'ProductSelection_N_ary::ProductSelection2::cart'[feature_def][implied]
          (multiplicity_range [1])
          (feature_def 'inCart1'
            (multiplicity_range [0..1])))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def] :>> 'ProductSelection_N_ary::ProductSelection2::selectedProduct'[feature_def][implied]
          (multiplicity_range [1])
          (feature_def 'selectedProducts1'
            (multiplicity_range [0..*])))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def] :>> 'ProductSelection_N_ary::ProductSelection2::account'[feature_def][implied]
          (multiplicity_range [1])
          (feature_def 'withAccount1'
            (multiplicity_range [0..*]))))
      (association_def 'SingleProductSelection3' :> 'ProductSelection_N_ary::ProductSelection3'[association_def]
        (feature_def end 'cart' : 'ProductSelection_N_ary::ShoppingCart'[class_def] :>> 'ProductSelection_N_ary::ProductSelection3::cart'[feature_def] :> 'cart::product_account1::inCart1'[unresolved]
          (multiplicity_range [1])
          (feature_def 'inCart1' : 'ProductSelection_N_ary::ShoppingCart'[class_def]
            (multiplicity_range [0..1])
            (feature_def 'Product_Account1' :> 'Product_Account'[unresolved] : 'ProductSelection_N_ary::Account'[class_def]))
          (feature_def 'product_account1' : 'ProductSelection_N_ary::SingleProductSelection3::cart::inCart1::Product_Account1'[feature_def]
            (membership_import public -> 'ProductSelection_N_ary::SingleProductSelection3::cart::inCart1'[feature_def])))
        (feature_def end 'selectedProduct' : 'ProductSelection_N_ary::Product'[class_def] :>> 'ProductSelection_N_ary::ProductSelection3::selectedProduct'[feature_def] :> 'selectedProduct::cart_account1::selectedProduct1'[unresolved]
          (multiplicity_range [1])
          (feature_def 'selectedProduct1' : 'ProductSelection_N_ary::Product'[class_def]
            (multiplicity_range [1..1])
            (feature_def 'Cart_Account1' :> 'Cart_Account'[unresolved] : 'ProductSelection_N_ary::Account'[class_def]))
          (feature_def 'cart_account1' : 'ProductSelection_N_ary::SingleProductSelection3::selectedProduct::selectedProduct1::Cart_Account1'[feature_def]
            (membership_import public -> 'ProductSelection_N_ary::SingleProductSelection3::selectedProduct::selectedProduct1'[feature_def])))
        (feature_def end 'account' : 'ProductSelection_N_ary::Account'[class_def] :> 'account::cart_product1::withAccount1'[unresolved] :>> 'ProductSelection_N_ary::ProductSelection3::account'[feature_def][implied]
          (multiplicity_range [1])
          (feature_def 'withAccount1' : 'ProductSelection_N_ary::Account'[class_def]
            (multiplicity_range [1..1])
            (feature_def 'Cart_Product1' :> 'Cart_Product'[unresolved] : 'ProductSelection_N_ary::Product'[class_def]))
          (feature_def 'cart_product1' : 'ProductSelection_N_ary::SingleProductSelection3::account::withAccount1::Cart_Product1'[feature_def]
            (membership_import public -> 'ProductSelection_N_ary::SingleProductSelection3::account::withAccount1'[feature_def]))))
      (class_def 'OnlineCustomer'
        (feature_def 'myCart' : 'ProductSelection_N_ary::ShoppingCart'[class_def]
          (multiplicity_range [1]))
        (feature_def 'products' : 'ProductSelection_N_ary::Product'[class_def]
          (multiplicity_range [0..*]))
        (feature_def 'myAccount' : 'ProductSelection_N_ary::Account'[class_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (not_implemented 'malformed')))))
~~~
