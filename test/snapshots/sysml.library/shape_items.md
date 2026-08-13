# META
~~~ini
description=Standard Library: Domain Libraries/Geometry/ShapeItems
type=file
~~~
# SOURCE
~~~sysml
standard library package ShapeItems {
	doc
	/*
	 * This package provides a model of items that represent basic geometric shapes. 
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Positive;
	private import ISQSpaceTime::*;
	private import ISQBase::*;
	private import SI::m;
	private import Occurrences::MatesWith;
	private import Objects::*;
	private import Items::Item;
	private import SequenceFunctions::equals;
	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::includes;
	private import ControlFunctions::'if';
	private import ControlFunctions::forAll;
	private import ControlFunctions::exists;
	private import Quantities::scalarQuantities;

	item def PlanarCurve :> Curve {
		doc
		/*
		 * A PlanarCurve is a Curve with a given length embeddable in a plane.
		 */
	
		attribute :>> length [1];

		attribute :>> outerSpaceDimension;
		assert constraint { notEmpty(outerSpaceDimension) &  outerSpaceDimension <= 2 }
	}

	item def PlanarSurface :> Surface {
		doc
		/*
		 * A PlanarSurface is a flat Surface with a given area.
		 */
	
		attribute :>> area [1];
		attribute :>> outerSpaceDimension = 2;

		item :>> shape : PlanarCurve;
	}

	item def Line :> PlanarCurve {
		doc
		/*
		 * A Line is a Curve that is a straight line of a given length.
		 */
	
		attribute :>> length [1];
		attribute :>> outerSpaceDimension = 1;
	}

	abstract item def Path :> StructuredSpaceObject::StructuredCurve {
		doc
		/*
		 * Path is the most general structured Curve.
		 */
        
		item :>> faces [0];
		item :>> edges [1..*] {
			item :>> vertices [0..2];
		}
		item :>> vertices [*] = edges.vertices;

		assert constraint { isClosed == vertices->forAll{in p1 : Point;
					vertices->exists{p2 : Point; p1 != p2 and
							 includes(p1.matingOccurrences, p2) } } }
	}

	attribute semiMajorAxis : LengthValue [0..*] :> scalarQuantities;
	attribute semiMinorAxis : LengthValue [0..*] :> scalarQuantities;
	attribute xoffset : LengthValue [0..*] :> scalarQuantities default 0 [m];
	attribute yoffset : LengthValue [0..*] :> scalarQuantities default 0 [m];
	attribute baseLength : LengthValue [0..*] :> scalarQuantities;
	attribute baseWidth : LengthValue [0..*] :> scalarQuantities;

	item def ConicSection :> Path, PlanarCurve {
		doc
		/*
		 * A ConicSection is a closed PlanarCurve, possibly disconnected, see Hyperbola.
		 */
	

		item :>> edges [1..2];

		item :>> vertices [0];
	}

	item def Ellipse :> ConicSection {
		doc
		/*
		 * An Ellipse is a ConicSection in the shape of an ellipse of a given semiaxes.
		 */
	
		attribute :>> semiMajorAxis [1];
		attribute :>> semiMinorAxis [1];

		item :>> edges [1];
	}

	item def Circle :> Ellipse {
		doc
		/*
		 * A Circle is an Ellipse with semiaxes equal to its radius.
		 */
	
		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> edges {
			attribute length [1] = Circle::radius * TrigFunctions::pi * 2;
		}
	}

	item def Parabola :> ConicSection {
		doc
		/*
		 * A Parabola is a ConicSection in the shape of a parabola of a given focal length.
		 */
	
		attribute focalDistance : LengthValue [1] :> scalarQuantities;

		item :>> edges [1];
	}

	item def Hyperbola :> ConicSection {
		doc
		/*
		 * A Hyperbola is a ConicSection in the shape of a hyperbola with given axes.
		 */
	
		attribute tranverseAxis : LengthValue [1] :> scalarQuantities;
		attribute conjugateAxis : LengthValue [1] :> scalarQuantities;
	}

	item def Polygon :> Path, PlanarCurve {
		doc
		/*
		 * A Polygon is a closed planar Path with straight edges.
		 */
	
		item :>> edges : Line { item :>> vertices [2]; }

		attribute :>> isClosed = true;

		assert constraint { (1..size(edges))->forAll {in i;
					edges#(i).vertices->equals((vertices#((2*i)-1), vertices#(2*i))) and  
					includes((edges#(i).vertices#(2) as Item).matingOccurrences,
						 edges#(if i==size(edges) ? 1 else i+1).vertices#(1)) } }
	}

	item def Triangle :> Polygon {
		doc
		/*
		 * A Triangle is three-sided Polygon  with given length (base), width (perpendicular distance
		 * from base to apex), and offset of this perpendicular at the base from the center of the base.
		 */
	
		attribute :>> length [1];
		attribute :>> width [1];
		attribute :>> xoffset [1];

		item :>> edges [3] = (base, e2, e3);
		item base [1] { length = Triangle::length; }
		item e2 [1];
		item e3 [1];

		item :>> vertices [6];
		item v12  [2] ordered = (vertices#(2), vertices#(3));
		item apex [2] ordered = (vertices#(4), vertices#(5));
		item v31  [2] ordered = (vertices#(6), vertices#(1));
	}

	item def RightTriangle :> Triangle {
		doc
		/*
		 * A RightTriangle is a Triangle with sides opposite the hypotenuse at right angles.
		 */
	
		attribute :>> xoffset = length / 2;

		item :>> e2 { attribute :>> length = Triangle::width; }

		item hypotenuse :>> e3 {
			attribute :>> length = ( Triangle::length^2 + Triangle::width^2 );
		}
	}

	item def Quadrilateral :> Polygon {
		doc
		/*
		 * A Quadrilateral is a four-sided Polygon.
		 */
	
		item :>> edges [4] = (e1, e2, e3, e4);
		item e1 [1];
		item e2 [1];
		item e3 [1];
		item e4 [1];

		item :>> vertices [8];
		item v12 [2] ordered = (vertices#(2), vertices#(3));
		item v23 [2] ordered = (vertices#(4), vertices#(5));
		item v34 [2] ordered = (vertices#(6), vertices#(7));
		item v41 [2] ordered = (vertices#(6), vertices#(1));
	}

	item def Rectangle :> Quadrilateral {
		doc
		/*
		 * A Rectangle is a Quadrilateral four right angles and given length and width.
		 */
	
		attribute :>> length [1];
		attribute :>> width [1];

		item :>> e1 { attribute :>> length = Rectangle::length; }
		item :>> e2 { attribute :>> length = Rectangle::width; }
		item :>> e3 { attribute :>> length = e1.length; }
		item :>> e4 { attribute :>> length = e2.length; }
	}

	abstract item def Shell :> StructuredSpaceObject::StructuredSurface {
		doc
		/*
		 * Shell is the most general structured Surface.
		 */
	}

	item def Disc :> Shell, PlanarSurface {
		doc
		/*
		 * A Disc is a Shell bound by an Ellipse.
		 */
	
		attribute :>> semiMajorAxis [1];
		attribute :>> semiMinorAxis [1];

		item :>> shape : Ellipse [1] {
			attribute :>> semiMajorAxis = Disc::semiMajorAxis;
			attribute :>> semiMinorAxis = Disc::semiMinorAxis;
		}

		item :>> faces : PlanarSurface [1] {
			item :>> edges [1];
		}
		item :>> edges : Ellipse [1] = shape {
            attribute :>> Shell::edges::innerSpaceDimension, Ellipse::innerSpaceDimension;
            ref item :>> Shell::edges::vertices, Ellipse::vertices;
		}
		item :>> vertices [0];
	}

	item def CircularDisc :> Disc {
		doc
		/*
		 * A CircularDisc is a Disc bound by a Circle.
		 */
	
		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> shape : Circle {
            attribute :>> Disc::shape::semiMajorAxis, Circle::semiMajorAxis;
            attribute :>> Disc::shape::semiMinorAxis, Circle::semiMinorAxis;
        }
		item :>> edges : Circle;
	}

	item def ConicSurface :> Shell {
		doc
		/*
		 * A ConicSurface is a Surface that has ConicSection cross-sections.
		 */
	
		item :>> faces [1..2];
		item :>> edges [0];
		item :>> vertices [0];

		attribute :>> genus = 0;
	}

	item def Ellipsoid :> ConicSurface {
		doc
		/*
		 * An Ellipsoid is a ConicSurface with only elliptical cross-sections.
		 */
	
		attribute semiAxis1 : LengthValue [1] :> scalarQuantities; 
		attribute semiAxis2 : LengthValue [1] :> scalarQuantities;
		attribute semiAxis3 : LengthValue [1] :> scalarQuantities;

		item :>> faces [1];
	}

	item def Sphere :> Ellipsoid {
		doc
		/*
		 * A Sphere is an Ellipsoid with all the same semiaxes.
		 */	

		attribute :>> radius [1];
		attribute :>> semiAxis1 [1] = radius;
		attribute :>> semiAxis2 [1] = radius;
		attribute :>> semiAxis3 [1] = radius;
	}

	item def Paraboloid :> ConicSurface {
		doc
		/*
		 * A Paraboloid is a ConicSurface with only parabolic cross-sections.
		 */
	
		attribute focalDistance : LengthValue [1] :> scalarQuantities;

		item :>> faces [1];
	}

	item def Hyperboloid :> ConicSurface {
		doc
		/*
		 * A Hyperboloid is a ConicSurface with only hyperbolic cross-sections.
		 */
	
		attribute transverseAxis : LengthValue [1] :> scalarQuantities;
		attribute conjugateAxis : LengthValue [1] :> scalarQuantities;
	}

	item def Toroid :> Shell {
		doc
		/*
		 * A Toroid is a surface generated from revolving a planar closed curve about an line coplanar
		 * with the curve. It is single sided with one hole.
		 */	

		attribute revolutionRadius : LengthValue [1] :> scalarQuantities;

		item revolvedCurve : PlanarCurve [1] { attribute :>> isClosed = true; }

		item :>> faces [1];
		item :>> edges [0];
		item :>> vertices [0];

		attribute :>> genus = 1;
	}

	item def Torus :> Toroid {
		doc
		/*
		 * A Torus is a revolution of a Circle.
		 */	

		attribute majorRadius :>> revolutionRadius;
		attribute minorRadius : LengthValue [1] :> scalarQuantities;

		item :>> revolvedCurve: Circle [1] { attribute :>> radius = minorRadius; }
	}


	item def RectangularToroid :> Toroid {
		doc
		/*
		 * A RectangularToroid is a revolution of a Rectangle.
		 */	

		attribute rectangleLength : LengthValue [1] :> scalarQuantities;
		attribute rectangleWidth  : LengthValue [1] :> scalarQuantities;

		item :>> revolvedCurve: Rectangle [1] {
			attribute :>> length = rectangleLength;
			attribute :>> width  = rectangleWidth;
			attribute :>> revolvedCurve::isClosed, Rectangle::isClosed;
		}
	}

	item def ConeOrCylinder :> Shell {
		doc
		/*
		 * A ConeOrCylinder is Shell that a Cone or a Cylinder with a given elliptical base,
		 * height, width (perpendicular distance from the base to the center of the top side or vertex),
		 * and offsets of this perpendicular at the base from the center of the base.
		 */
	
		attribute :>> semiMajorAxis [1];
		attribute :>> semiMinorAxis [1];
		attribute :>> height [1];

		attribute :>> xoffset [1];
		attribute :>> yoffset [1];

		item :>> faces [2..3];
		item base : Disc [1] :> faces {        
            attribute :>> Disc::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Disc::edges, ConeOrCylinder::faces::edges {
                attribute :>> Disc::edges::innerSpaceDimension, ConeOrCylinder::faces::edges::innerSpaceDimension;
            }
            ref :>> Disc::vertices, ConeOrCylinder::faces::vertices;		    
		}
		item af : Disc [0..1] :> faces {        
            attribute :>> Disc::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Disc::edges, ConeOrCylinder::faces::edges {
                attribute :>> Disc::edges::innerSpaceDimension, ConeOrCylinder::faces::edges::innerSpaceDimension;
            }
            ref :>> Disc::vertices, ConeOrCylinder::faces::vertices;            
        }
		item cf : Surface [1] :> faces;

		item :>> edges [2..4] = faces.edges;
		item be [2] :> edges { 
			attribute :>> semiMajorAxis = ConeOrCylinder::semiMajorAxis;
			attribute :>> semiMinorAxis = ConeOrCylinder::semiMinorAxis;
		}
		item ae [0..2] :> edges {
			attribute :>> semiMajorAxis = be.semiMajorAxis;
			attribute :>> semiMinorAxis = be.semiMinorAxis;
		}
		assert constraint { size(ae) == (if isEmpty(af) ? 0 else 2) and
				            size(edges) == (if isEmpty(af) ? 2 else 4)  }

		item :>> vertices [0..1] = faces.vertices;
		assert constraint { isEmpty(af) == notEmpty(vertices) }

		/* Bind face edges to specific edges */
		binding [1] bind [0..*] base.edges = [0..*] be;
		binding [1] bind [0..*] cf.edges = [0..*] be;

		/* Meeting edges */
		connection :MatesWith connect [1] be to [1] be;

		attribute :>> genus = 0;
	}

	item def Cone :> ConeOrCylinder {
		doc
		/*
		 * A Cone has one elliptical sides joined to a point by a curved side.
		 */	

		item :>> faces [2];

		item apex :>> vertices;

		/* Bind face vertices to specific vertices */
		binding [1] bind [0..*] cf.vertices = [0..*] apex;
	}

	item def EccentricCone :> Cone {
		doc
		/*
		 * An EccentricCone is a Cone with least one positive offset.
		 */
	
		assert constraint { xoffset > 0 or yoffset > 0 }
	}

	item def CircularCone :> Cone {
		doc
		/*
		 * A CircularCone is a Cone with a circular base.
		 */	

		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> base : CircularDisc {
		    ref :>> base::edges, CircularDisc::edges;
		}
	}

	item def RightCircularCone :> CircularCone {
		doc
		/*
		 * A RightCircularCone is a CircularCone with zero offsets.
		 */
	
		attribute :>> xoffset { attribute :>> num = 0; }
		attribute :>> yoffset { attribute :>> num = 0; }
	}

	item def Cylinder :> ConeOrCylinder {
		doc
		/*
		 * A Cylinder has two elliptical sides joined by a curved side.
		 */
	
		item :>> af [1];

		binding [1] bind [0..*] cf.edges = [0..*] ae;

		connection :MatesWith connect [1] ae to [1] ae {
			doc /* Meeting edges */
		}
	}

	item def EccentricCylinder :> Cylinder {
	doc
	/*
	 * An EccentricCylinder is a Cylinder with least one positive offset.
	 */
	
		assert constraint { xoffset > 0 or yoffset > 0 }
	}

	item def CircularCylinder :> Cylinder {
		doc
		/*
		 * A CircularCylinder is a Cylinder with two circular sides.
		 */
	
		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> base : CircularDisc {
            ref :>> base::edges, CircularDisc::edges;
        }
		item :>> af : CircularDisc {
            ref :>> af::edges, CircularDisc::edges;
        }
	}

	item def RightCircularCylinder :> CircularCylinder {
		doc
		/*
		 * A RightCircularCylinder is a CircularCylinder with zero offsets.
		 */
	
		attribute :>> xoffset { attribute :>> num = 0; }
		attribute :>> yoffset { attribute :>> num = 0; }
	}

	item def Polyhedron :> Shell {
		doc
		/*
		 * A Polyhedron is a closed Shell with polygonal sides.
		 */	

		attribute :>> isClosed = true;

		item :>> faces : Polygon [2..*] {        
            attribute :>> Polygon::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Polygon::edges, ConeOrCylinder::faces::edges;
            ref :>> Polygon::vertices, ConeOrCylinder::faces::vertices;            
        }
		
		item :>> edges = faces.edges;
		
		attribute :>> outerSpaceDimension = if size(faces) > 2 ? 3 else 2;

		attribute :>> genus = 0;
	}

	item def CuboidOrTriangularPrism :> Polyhedron {
		doc
		/*
		 * A CuboidOrTriangularPrism is a Polyhedron that is either a Cuboid or TriangularPrism.
		 */

		item :>> faces [5..6];
		item tf	 : Quadrilateral [1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item bf	 : Quadrilateral [1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item ff	 : Polygon [1] :> faces { item :>> Polygon::edges, faces::edges [3..4]; }
		item rf	 : Polygon [1] :> faces { item :>> Polygon::edges, faces::edges [3..4]; }
		item slf : Quadrilateral [1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item srf : Quadrilateral [0..1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }

		item :>> edges;
		assert constraint { size(edges) == 18 or size(edges) == 24 }
		
		item tfe  [2]	 :> edges;
		item tre  [2]	 :> edges;
		item tsle [2]	 :> edges;
		item tsre [0..2] :> edges;
		item bfe  [2]	 :> edges;
		item bre  [2]	 :> edges;
		item bsle [2]	 :> edges;
		item bsre [2]	 :> edges;
		item ufle [2]	 :> edges;
		item ufre [0..2] :> edges;
		item urle [2]	 :> edges;
		item urre [0..2] :> edges;

		assert constraint { ( isEmpty(srf) implies isEmpty(tsre) ) and
				    ( isEmpty(tsre) == isEmpty(ufre) ) and
				    ( isEmpty(ufre) == isEmpty(urre) ) }

		item :>> vertices;
		assert constraint { size(vertices) == size(edges) }

		item tflv [3]	 :> vertices;
		item tfrv [0..3] :> vertices;
		item trlv [3]	 :> vertices;
		item trrv [0..3] :> vertices;
		item bflv [3]	 :> vertices;
		item bfrv [3]	 :> vertices;
		item brlv [3]	 :> vertices;
		item brrv [3]	 :> vertices;
		
		assert constraint { ( isEmpty(tfrv) == isEmpty(trrv) ) }

		/* Bind face edges to specific edges */
		binding [1] bind [0..1] tf.edges = [0..1] tfe;
		binding [1] bind [0..1] tf.edges = [0..1] tre;
		binding [1] bind [0..1] tf.edges = [0..1] tsle;
		binding [1] bind [0..1] bf.edges = [0..1] bfe;
		binding [1] bind [0..1] bf.edges = [0..1] bre;
		binding [1] bind [0..1] bf.edges = [0..1] bsle;
		binding [1] bind [0..1] bf.edges = [0..1] bsre;

		binding [1] bind [0..1] ff.edges = [0..1] tfe;
		binding [1] bind [0..1] ff.edges = [0..1] bfe;
		binding [1] bind [0..1] ff.edges = [0..1] ufle;

		binding [1] bind [0..1] rf.edges = [0..1] tre;
		binding [1] bind [0..1] rf.edges = [0..1] bre;
		binding [1] bind [0..1] rf.edges = [0..1] urle;

		/* Bind edge vertices to specific vertices */
		binding [1] bind [0..1] tfe.vertices = [0..1] tflv;
		binding [1] bind [0..1] tre.vertices = [0..1] trlv;
		binding [1] bind [0..1] tsle.vertices = [0..1] tflv;
		binding [1] bind [0..1] tsle.vertices = [0..1] trlv;

		binding [1] bind [0..1] bfe.vertices = [0..1] bflv;
		binding [1] bind [0..1] bfe.vertices = [0..1] bfrv;
		binding [1] bind [0..1] bre.vertices = [0..1] brlv;
		binding [1] bind [0..1] bre.vertices = [0..1] brrv;
		binding [1] bind [0..1] bsle.vertices = [0..1] bflv;
		binding [1] bind [0..1] bsle.vertices = [0..1] brlv;
		binding [1] bind [0..1] bsre.vertices = [0..1] bfrv;
		binding [1] bind [0..1] bsre.vertices = [0..1] brrv;

		binding [1] bind [0..1] ufle.vertices = [0..1] tflv;
		binding [1] bind [0..1] ufle.vertices = [0..1] bflv;
		binding [1] bind [0..1] urle.vertices = [0..1] trlv;
		binding [1] bind [0..1] urle.vertices = [0..1] brlv;

		/* Meeting edges */
		connection :MatesWith connect [1] tfe to [1] tfe;
		connection :MatesWith connect [1] tre to [1] tre;
		connection :MatesWith connect [1] tsle to [1] tsle;
		connection :MatesWith connect [1] bfe to [1] bfe;
		connection :MatesWith connect [1] bre to [1] bre;
		connection :MatesWith connect [1] bsle to [1] bsle;
		connection :MatesWith connect [1] bsre to [1] bsre;
		connection :MatesWith connect [1] ufle to [1] ufle;
		connection :MatesWith connect [1] urle to [1] urle;
		connection :MatesWith connect [1] bsre to [1] bsre;

		/* Meeting vertices  */
		connection :MatesWith connect [2] tflv to [2] tflv;
		connection :MatesWith connect [2] trlv to [2] trlv;
		connection :MatesWith connect [2] bflv to [2] bflv;
		connection :MatesWith connect [2] bfrv to [2] bfrv;
		connection :MatesWith connect [2] brlv to [2] brlv;
		connection :MatesWith connect [2] brrv to [2] brrv;
	}

	item def TriangularPrism :> CuboidOrTriangularPrism {
		doc
		/*
		 * A TriangularPrism is a Polyhedron with five sides, two triangular and
		 * the others quadrilateral.
		 */
	

		item :>> faces [5];
		item :>> ff : Triangle {        
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
        }
		item :>> rf : Triangle {        
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
        }

		item :>> edges [18];

		item :>> vertices;

		/* Bind face edges to specific edges */
		binding [1] bind [0..1] tf.edges = [0..1] bsre;

		/* Bind edge vertices to specific vertices */
		binding [1] bind [0..1] tfe.vertices = [0..1] bfrv;
		binding [1] bind [0..1] tre.vertices = [0..1] bfrv;
	}

	item def RightTriangularPrism :> TriangularPrism {
		doc
		/*
		 * A RightTriangularPrism  a TriangularPrism with two right triangluar sides,
		 * with given length, width, and height.
		 */
	 
		attribute :>> length [1];
		attribute :>> width [1];
		attribute :>> height [1];

		item :>> tf  : Rectangle;
		item :>> bf  : Rectangle;
		item :>> ff : RightTriangle {
			attribute :>> length = RightTriangularPrism::length;
			attribute :>> width = RightTriangularPrism::width;
		}
		item :>> rf : RightTriangle {
			attribute :>> length = ff.length;
			attribute :>> width = rf.width;
		}
		item :>> slf : Rectangle;
		item :>> srf : Rectangle;

		item :>> tfe  { attribute :>> length = ff.hypotenuse.length; }
		item :>> tre  { attribute :>> length = tfe.length; }
		item :>> tsle { attribute :>> length = height; }
		item :>> bfe  { attribute :>> length = RightTriangularPrism::length; }
		item :>> bre  { attribute :>> length = RightTriangularPrism::length; }
		item :>> bsle { attribute :>> length = height; }
		item :>> bsre { attribute :>> length = height; }
		item :>> ufle { attribute :>> length = width;  } 
		item :>> urle { attribute :>> length = width; }
	}
	alias Wedge for RightTriangularPrism;

	item def Cuboid :> CuboidOrTriangularPrism {
		doc
		/*
		 * A Cuboid is a Polyhedron with six sides, all quadrilateral.
		 */	

		item :>> faces [6];
		item :>> ff : Quadrilateral {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item :>> rf : Quadrilateral {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }

		item :>> edges [24];

		item :>> vertices;

		/* Bind face edges to specific edges */
		binding [1] bind [0..1] tf.edges = [0..1] tsre;
		binding [1] bind [0..1] ff.edges = [0..1] ufre;
		binding [1] bind [0..1] rf.edges = [0..1] urre;

		binding [1] bind [0..1] srf.edges = [0..1] tsre;
		binding [1] bind [0..1] srf.edges = [0..1] bsre;
		binding [1] bind [0..1] srf.edges = [0..1] ufre;
		binding [1] bind [0..1] srf.edges = [0..1] urre;

		/* Bind edge vertices to specific vertices */
		binding [1] bind [0..1] tfe.vertices = [0..1] tfrv;
		binding [1] bind [0..1] tre.vertices = [0..1] trrv;
		binding [1] bind [0..1] tsre.vertices = [0..1] tfrv;
		binding [1] bind [0..1] tsre.vertices = [0..1] trrv;

		binding [1] bind [0..1] ufre.vertices = [0..1] tfrv;
		binding [1] bind [0..1] ufre.vertices = [0..1] bfrv;
		binding [1] bind [0..1] urre.vertices = [0..1] trrv;
		binding [1] bind [0..1] urre.vertices = [0..1] brrv;

		/* Meeting edges */
		connection :MatesWith connect [1] tsre to [1] tsre;
		connection :MatesWith connect [1] ufre to [1] ufre;
		connection :MatesWith connect [1] urre to [1] urre;
		connection :MatesWith connect [1] bsre to [1] bsre;

		/* Meeting vertices  */
		connection :MatesWith connect [2] tfrv to [2] tfrv;
		connection :MatesWith connect [2] trrv to [2] trrv;
	}

	item def RectangularCuboid :> Cuboid {
		doc
		/*
		 * A RectangularCuboid is a Cuboid with all Rectangular sides.
		 */
	
		attribute :>> length [1];
		attribute :>> width [1];
		attribute :>> height [1];
	
		item :>> tf  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::height; }
		item :>> bf  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::height; }
		item :>> ff  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::width; }
		item :>> rf  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::width; }
		item :>> slf : Rectangle { attribute :>> length = RectangularCuboid::height;
								   attribute :>> width	= RectangularCuboid::width; }
		item :>> srf : Rectangle { attribute :>> length = RectangularCuboid::height;
								   attribute :>> width	= RectangularCuboid::width; }
	}
	alias Box for RectangularCuboid;

	item def Pyramid :> Polyhedron {
		doc
		/*
		 * A Pyramid is a Polyhedron with the sides of a polygon (base) forming the bases of triangles
		 * that join at an apex point.	Its height is the perpendicular distance from the base to the apex,
		 * and its offsets are between this perpendicular at the base and the center of the base.
		 */	 

		attribute :>> height [1];
		attribute :>> xoffset;
		attribute :>> yoffset;

		item :>> faces;
		item base [1] :> faces;
		item wall : Triangle :> faces {        
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
        }
		attribute wallNumber : Positive = size(wall);

		assert constraint { size(faces) == wallNumber + 1 }
		assert constraint { size(wall) == size(base.edges) }

		item :>> edges;

		assert constraint { size(edges) == wallNumber * 4 }

		item :>> vertices;
		item apex :> vertices = wall.apex;

		assert constraint { size(apex) == wallNumber }

		/* Base to wall and wall to wall edge mating. */
		assert constraint { (1..wallNumber)->forAll {in i;
					includes(wall#(i).base.matingOccurrences,
							 Pyramid::base.edges#(i)) and
					includes((wall#(i).edges#(3) as Item).matingOccurrences,
							 wall#(if i==wallNumber ? 1 else i+1).edges#(2)) } }

		/* Meeting apices. */
		connection :MatesWith connect [wallNumber] apex to [wallNumber] apex;
	}

	item def Tetrahedron :> Pyramid {
		doc
		/*
		 * A Tetrahedron is Pyramid with a triangular base.
		 */
	
		attribute :>> baseLength [1];
		attribute :>> baseWidth [1];

		item :>> base : Triangle {
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
			attribute :>> length = Tetrahedron::baseLength;
			attribute :>> width  = Tetrahedron::baseWidth;
		}
	}

	item def RectangularPyramid :> Pyramid {
		doc
		/*
		 * A RectangularPyramid is Pyramid with a rectangular base.
		 */	

		attribute :>> baseLength [1];
		attribute :>> baseWidth [1];

		item :>> base : Rectangle {
            ref :>> Rectangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Rectangle::vertices, ConeOrCylinder::faces::vertices;            
			attribute :>> length = RectangularPyramid::baseLength;
			attribute :>> width = RectangularPyramid::baseWidth;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/shape_items.md"
    (diagnostics
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
        (range (start 7 16) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 16) (end 22 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 24 25) (end 24 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 16) (end 30 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 16) (end 32 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 33 2) (end 33 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 27) (end 36 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 16) (end 42 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 16) (end 43 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 45 2) (end 45 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 16) (end 54 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 16) (end 55 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 58 27) (end 58 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 64 2) (end 64 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 65 2) (end 67 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 68 2) (end 68 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 70 2) (end 72 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 27) (end 75 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 27) (end 76 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 21) (end 77 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 21) (end 78 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 79 24) (end 79 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 23) (end 80 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 89 2) (end 89 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 91 2) (end 91 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 103 2) (end 103 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 16) (end 112 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 116 2) (end 118 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 28) (end 127 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 127 47) (end 127 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 129 2) (end 129 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 28) (end 138 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 138 47) (end 138 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 28) (end 139 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 139 47) (end 139 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 148 2) (end 148 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 150 16) (end 150 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 152 2) (end 155 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 165 16) (end 165 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 166 16) (end 166 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 169 2) (end 169 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 170 2) (end 170 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 171 2) (end 171 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 172 2) (end 172 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 174 2) (end 174 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 175 2) (end 175 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 176 2) (end 176 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 177 2) (end 177 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 188 2) (end 188 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 190 2) (end 192 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 201 2) (end 201 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 202 2) (end 202 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 203 2) (end 203 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 204 2) (end 204 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 205 2) (end 205 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 207 2) (end 207 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 208 2) (end 208 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 209 2) (end 209 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 210 2) (end 210 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 211 2) (end 211 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 220 16) (end 220 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 221 16) (end 221 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 223 2) (end 223 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 224 2) (end 224 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 225 2) (end 225 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 226 2) (end 226 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 229 28) (end 229 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 245 2) (end 248 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 250 2) (end 252 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 253 2) (end 256 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 257 2) (end 257 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 266 16) (end 266 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 270 2) (end 273 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 274 2) (end 274 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 283 2) (end 283 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 284 2) (end 284 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 285 2) (end 285 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 287 16) (end 287 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 296 24) (end 296 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 296 43) (end 296 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 297 24) (end 297 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 297 43) (end 297 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 298 24) (end 298 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 298 43) (end 298 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 300 2) (end 300 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 309 16) (end 309 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 28) (end 321 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 321 47) (end 321 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 323 2) (end 323 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 332 29) (end 332 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 332 48) (end 332 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 28) (end 333 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 333 47) (end 333 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 343 31) (end 343 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 343 50) (end 343 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 345 2) (end 345 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 347 2) (end 347 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 348 2) (end 348 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 349 2) (end 349 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 351 16) (end 351 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 361 26) (end 361 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 361 45) (end 361 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 363 2) (end 363 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 373 30) (end 373 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 373 49) (end 373 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 374 30) (end 374 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 374 49) (end 374 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 376 2) (end 380 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 393 16) (end 393 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 398 2) (end 398 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 399 2) (end 405 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 406 2) (end 412 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 413 2) (end 413 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 415 2) (end 415 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 416 2) (end 419 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 420 2) (end 423 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 424 2) (end 425 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 427 2) (end 427 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 428 2) (end 428 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 431 2) (end 431 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 432 2) (end 432 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 435 2) (end 435 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 437 16) (end 437 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 446 2) (end 446 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 448 2) (end 448 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 451 2) (end 451 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 460 2) (end 460 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 469 16) (end 469 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 473 2) (end 475 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 484 40) (end 484 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 485 40) (end 485 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 494 2) (end 494 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 496 2) (end 496 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 498 2) (end 500 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 509 2) (end 509 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 518 16) (end 518 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 522 2) (end 524 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 525 2) (end 527 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 536 40) (end 536 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 537 40) (end 537 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 546 16) (end 546 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 548 2) (end 552 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 554 2) (end 554 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 556 16) (end 556 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 558 16) (end 558 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 567 2) (end 567 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 568 2) (end 571 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 572 2) (end 575 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 576 2) (end 576 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 577 2) (end 577 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 578 2) (end 581 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 582 2) (end 585 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 587 2) (end 587 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 588 2) (end 588 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 590 2) (end 590 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 591 2) (end 591 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 592 2) (end 592 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 593 2) (end 593 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 594 2) (end 594 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 595 2) (end 595 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 596 2) (end 596 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 597 2) (end 597 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 598 2) (end 598 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 599 2) (end 599 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 600 2) (end 600 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 601 2) (end 601 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 603 2) (end 605 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 607 2) (end 607 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 608 2) (end 608 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 610 2) (end 610 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 611 2) (end 611 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 612 2) (end 612 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 613 2) (end 613 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 614 2) (end 614 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 615 2) (end 615 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 616 2) (end 616 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 617 2) (end 617 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 619 2) (end 619 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 622 2) (end 622 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 623 2) (end 623 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 624 2) (end 624 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 625 2) (end 625 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 626 2) (end 626 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 627 2) (end 627 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 628 2) (end 628 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 630 2) (end 630 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 631 2) (end 631 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 632 2) (end 632 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 634 2) (end 634 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 635 2) (end 635 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 636 2) (end 636 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 639 2) (end 639 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 640 2) (end 640 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 641 2) (end 641 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 642 2) (end 642 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 644 2) (end 644 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 645 2) (end 645 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 646 2) (end 646 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 647 2) (end 647 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 648 2) (end 648 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 649 2) (end 649 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 650 2) (end 650 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 651 2) (end 651 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 653 2) (end 653 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 654 2) (end 654 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 655 2) (end 655 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 656 2) (end 656 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 659 2) (end 659 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 660 2) (end 660 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 661 2) (end 661 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 662 2) (end 662 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 663 2) (end 663 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 664 2) (end 664 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 665 2) (end 665 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 666 2) (end 666 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 667 2) (end 667 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 668 2) (end 668 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 671 2) (end 671 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 672 2) (end 672 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 673 2) (end 673 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 674 2) (end 674 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 675 2) (end 675 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 676 2) (end 676 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 687 2) (end 687 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 688 2) (end 691 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 692 2) (end 695 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 697 2) (end 697 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 699 2) (end 699 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 702 2) (end 702 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 705 2) (end 705 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 706 2) (end 706 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 716 16) (end 716 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 717 16) (end 717 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 718 16) (end 718 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 720 2) (end 720 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 721 2) (end 721 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 722 2) (end 725 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 726 2) (end 729 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 730 2) (end 730 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 731 2) (end 731 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 733 2) (end 733 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 734 2) (end 734 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 735 2) (end 735 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 736 2) (end 736 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 737 2) (end 737 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 738 2) (end 738 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 739 2) (end 739 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 740 2) (end 740 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 741 2) (end 741 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 751 2) (end 751 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 752 2) (end 755 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 756 2) (end 759 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 761 2) (end 761 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 763 2) (end 763 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 766 2) (end 766 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 767 2) (end 767 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 768 2) (end 768 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 770 2) (end 770 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 771 2) (end 771 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 772 2) (end 772 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 773 2) (end 773 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 776 2) (end 776 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 777 2) (end 777 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 778 2) (end 778 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 779 2) (end 779 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 781 2) (end 781 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 782 2) (end 782 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 783 2) (end 783 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 784 2) (end 784 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 787 2) (end 787 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 788 2) (end 788 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 789 2) (end 789 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 790 2) (end 790 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 793 2) (end 793 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 794 2) (end 794 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 803 16) (end 803 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 804 16) (end 804 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 805 16) (end 805 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 807 2) (end 808 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 809 2) (end 810 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 811 2) (end 812 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 813 2) (end 814 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 815 2) (end 816 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 817 2) (end 818 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 830 16) (end 830 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 834 2) (end 834 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 835 2) (end 835 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 836 2) (end 839 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 840 25) (end 840 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 842 2) (end 842 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 843 2) (end 843 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 845 2) (end 845 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 847 2) (end 847 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 849 2) (end 849 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 850 2) (end 850 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 852 2) (end 852 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 855 2) (end 859 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 862 2) (end 862 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 874 2) (end 879 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 891 2) (end 896 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c61a35071bf088fb2aa947f398cf3dff11c93566a3dce6b885ca75bb30ac1f3a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQSpaceTime") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQBase") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::m") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::MatesWith") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Objects") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Items::Item") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::equals") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::includes") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::if") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 14))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 15))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::exists") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 16))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::scalarQuantities") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Box"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "RectangularCuboid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Circle"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Ellipse"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Cone"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Cylinder"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularDisc"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Disc"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConeOrCylinder"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Shell"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "height"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "xoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 4))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "yoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 5))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "genus"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Path")) (specialization (reference "PlanarCurve"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Shell"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "genus"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CuboidOrTriangularPrism"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Polyhedron"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConeOrCylinder"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Shell")) (specialization (reference "PlanarSurface"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCone"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Cone"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCylinder"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Cylinder"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConicSection"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMajorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiMinorAxis"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConicSurface"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConicSection"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConicSurface"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Line"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PlanarCurve"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "outerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConicSection"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConicSurface"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StructuredSpaceObject::StructuredCurve"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Curve"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "outerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarSurface"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Surface"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "area"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "outerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Path")) (specialization (reference "PlanarCurve"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isClosed"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Shell"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isClosed"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "outerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "genus"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Polyhedron"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "height"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "xoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "yoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Positive"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Polygon"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Rectangle"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Quadrilateral"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Cuboid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "height"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularPyramid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Pyramid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseLength"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseWidth"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Toroid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCone"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CircularCone"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "xoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "yoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CircularCylinder"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "xoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "yoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangle"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Triangle"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "xoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TriangularPrism"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "height"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StructuredSpaceObject::StructuredSurface"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Sphere"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Ellipsoid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiAxis1"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiAxis2"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "semiAxis3"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Tetrahedron"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Pyramid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseLength"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseWidth"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Shell"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "genus"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Toroid"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "revolutionRadius"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (subsetting (reference "scalarQuantities"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Polygon"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "xoffset"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CuboidOrTriangularPrism"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Wedge"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "RightTriangularPrism"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQSpaceTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQBase")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::MatesWith")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Items::Item")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::equals")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::if")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::exists")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Box"))) (kind aliasBinding) (ordinal 0))
      (authored-target "RectangularCuboid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Circle"))) (kind specialization) (ordinal 0))
      (authored-target "Ellipse")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone"))) (kind specialization) (ordinal 0))
      (authored-target "Cone")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder"))) (kind specialization) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularDisc"))) (kind specialization) (ordinal 0))
      (authored-target "Disc")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (kind specialization) (ordinal 0))
      (authored-target "ConeOrCylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind specialization) (ordinal 0))
      (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "height")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 4))))) (kind redefinition) (ordinal 0))
      (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 5))))) (kind redefinition) (ordinal 0))
      (authored-target "genus")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 0))
      (authored-target "Path")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 1))
      (authored-target "PlanarCurve")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (kind specialization) (ordinal 0))
      (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "genus")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid"))) (kind specialization) (ordinal 0))
      (authored-target "CuboidOrTriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind specialization) (ordinal 0))
      (authored-target "Polyhedron")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (kind specialization) (ordinal 0))
      (authored-target "ConeOrCylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 0))
      (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 1))
      (authored-target "PlanarSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarSurface")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCone"))) (kind specialization) (ordinal 0))
      (authored-target "Cone")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCylinder"))) (kind specialization) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse"))) (kind specialization) (ordinal 0))
      (authored-target "ConicSection")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid"))) (kind specialization) (ordinal 0))
      (authored-target "ConicSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola"))) (kind specialization) (ordinal 0))
      (authored-target "ConicSection")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid"))) (kind specialization) (ordinal 0))
      (authored-target "ConicSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Line"))) (kind specialization) (ordinal 0))
      (authored-target "PlanarCurve")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola"))) (kind specialization) (ordinal 0))
      (authored-target "ConicSection")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid"))) (kind specialization) (ordinal 0))
      (authored-target "ConicSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path"))) (kind specialization) (ordinal 0))
      (authored-target "StructuredSpaceObject::StructuredCurve")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve"))) (kind specialization) (ordinal 0))
      (authored-target "Curve")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarSurface"))) (kind specialization) (ordinal 0))
      (authored-target "Surface")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "area")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 0))
      (authored-target "Path")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 1))
      (authored-target "PlanarCurve")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isClosed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (kind specialization) (ordinal 0))
      (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isClosed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "genus")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (kind specialization) (ordinal 0))
      (authored-target "Polyhedron")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "height")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral"))) (kind specialization) (ordinal 0))
      (authored-target "Polygon")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Rectangle"))) (kind specialization) (ordinal 0))
      (authored-target "Quadrilateral")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid"))) (kind specialization) (ordinal 0))
      (authored-target "Cuboid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "height")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularPyramid"))) (kind specialization) (ordinal 0))
      (authored-target "Pyramid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "baseLength")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseWidth")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid"))) (kind specialization) (ordinal 0))
      (authored-target "Toroid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCone"))) (kind specialization) (ordinal 0))
      (authored-target "CircularCone")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind specialization) (ordinal 0))
      (authored-target "CircularCylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangle"))) (kind specialization) (ordinal 0))
      (authored-target "Triangle")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind specialization) (ordinal 0))
      (authored-target "TriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "height")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (kind specialization) (ordinal 0))
      (authored-target "StructuredSpaceObject::StructuredSurface")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Sphere"))) (kind specialization) (ordinal 0))
      (authored-target "Ellipsoid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "semiAxis1")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "semiAxis2")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "semiAxis3")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Tetrahedron"))) (kind specialization) (ordinal 0))
      (authored-target "Pyramid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "baseLength")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseWidth")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (kind specialization) (ordinal 0))
      (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "genus")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus"))) (kind specialization) (ordinal 0))
      (authored-target "Toroid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind redefinition) (ordinal 0))
      (authored-target "revolutionRadius")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind subsetting) (ordinal 0))
      (authored-target "scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle"))) (kind specialization) (ordinal 0))
      (authored-target "Polygon")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism"))) (kind specialization) (ordinal 0))
      (authored-target "CuboidOrTriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Wedge"))) (kind aliasBinding) (ordinal 0))
      (authored-target "RightTriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism")))))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Box"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Box"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Circle"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Circle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularDisc"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularDisc"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 4))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 4))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarSurface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCone"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCone"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCylinder"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Line"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Line"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Rectangle"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Rectangle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularPyramid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularPyramid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCone"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCone"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCylinder"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangle"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Sphere"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Sphere"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Tetrahedron"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Tetrahedron"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::majorRadius"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism"))) (kind specialization) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Wedge"))) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Wedge"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/shape_items.md") (range (start 8 16) (end 8 31)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQSpaceTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 9 16) (end 9 26)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQBase")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 12 16) (end 12 26)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0) (authored-target "Objects")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 6 16) (end 6 37)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 7 16) (end 7 38)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 10 16) (end 10 21)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SI::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 11 16) (end 11 38)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::MatesWith")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 13 16) (end 13 27)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Items::Item")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 14 16) (end 14 41)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 15 16) (end 15 42)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 16 16) (end 16 43)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 17 16) (end 17 39)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 18 16) (end 18 43)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 19 16) (end 19 38)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::if")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 20 16) (end 20 40)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 21 16) (end 21 40)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::exists")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 22 16) (end 22 44)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 820 15) (end 820 32)) (probe (position 820 15))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Box"))) (kind aliasBinding) (ordinal 0) (authored-target "RectangularCuboid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 106 20) (end 106 27)) (probe (position 106 20))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Circle"))) (kind specialization) (ordinal 0) (authored-target "Ellipse")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 112 16) (end 112 22)) (probe (position 112 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 113 16) (end 113 29)) (probe (position 113 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 114 16) (end 114 29)) (probe (position 114 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 463 26) (end 463 30)) (probe (position 463 26))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone"))) (kind specialization) (ordinal 0) (authored-target "Cone")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 469 16) (end 469 22)) (probe (position 469 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 470 16) (end 470 29)) (probe (position 470 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 471 16) (end 471 29)) (probe (position 471 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 512 30) (end 512 38)) (probe (position 512 30))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder"))) (kind specialization) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 518 16) (end 518 22)) (probe (position 518 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 519 16) (end 519 29)) (probe (position 519 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 520 16) (end 520 29)) (probe (position 520 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 260 26) (end 260 30)) (probe (position 260 26))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularDisc"))) (kind specialization) (ordinal 0) (authored-target "Disc")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 266 16) (end 266 22)) (probe (position 266 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 267 16) (end 267 29)) (probe (position 267 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 268 16) (end 268 29)) (probe (position 268 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 440 18) (end 440 32)) (probe (position 440 18))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone"))) (kind specialization) (ordinal 0) (authored-target "ConeOrCylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 383 28) (end 383 33)) (probe (position 383 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind specialization) (ordinal 0) (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 391 16) (end 391 29)) (probe (position 391 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 392 16) (end 392 29)) (probe (position 392 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 393 16) (end 393 22)) (probe (position 393 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "height")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 395 16) (end 395 23)) (probe (position 395 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 396 16) (end 396 23)) (probe (position 396 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 4))))) (kind redefinition) (ordinal 0) (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 437 16) (end 437 21)) (probe (position 437 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 5))))) (kind redefinition) (ordinal 0) (authored-target "genus")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 82 26) (end 82 30)) (probe (position 82 26))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 0) (authored-target "Path")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 82 32) (end 82 43)) (probe (position 82 32))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 1) (authored-target "PlanarCurve")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 277 26) (end 277 31)) (probe (position 277 26))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface"))) (kind specialization) (ordinal 0) (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 287 16) (end 287 21)) (probe (position 287 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "genus")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 745 20) (end 745 43)) (probe (position 745 20))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid"))) (kind specialization) (ordinal 0) (authored-target "CuboidOrTriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 561 37) (end 561 47)) (probe (position 561 37))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind specialization) (ordinal 0) (authored-target "Polyhedron")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 488 22) (end 488 36)) (probe (position 488 22))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder"))) (kind specialization) (ordinal 0) (authored-target "ConeOrCylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConeOrCylinder")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 236 18) (end 236 23)) (probe (position 236 18))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 0) (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 236 25) (end 236 38)) (probe (position 236 25))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 1) (authored-target "PlanarSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarSurface")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 242 16) (end 242 29)) (probe (position 242 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 243 16) (end 243 29)) (probe (position 243 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 454 27) (end 454 31)) (probe (position 454 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCone"))) (kind specialization) (ordinal 0) (authored-target "Cone")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cone")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 503 31) (end 503 39)) (probe (position 503 31))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::EccentricCylinder"))) (kind specialization) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cylinder")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 94 21) (end 94 33)) (probe (position 94 21))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipse"))) (kind specialization) (ordinal 0) (authored-target "ConicSection")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 100 16) (end 100 29)) (probe (position 100 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 101 16) (end 101 29)) (probe (position 101 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 290 23) (end 290 35)) (probe (position 290 23))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid"))) (kind specialization) (ordinal 0) (authored-target "ConicSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 296 24) (end 296 35)) (probe (position 296 24))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 296 43) (end 296 59)) (probe (position 296 43))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 297 24) (end 297 35)) (probe (position 297 24))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 297 43) (end 297 59)) (probe (position 297 43))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 298 24) (end 298 35)) (probe (position 298 24))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 298 43) (end 298 59)) (probe (position 298 43))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 132 23) (end 132 35)) (probe (position 132 23))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola"))) (kind specialization) (ordinal 0) (authored-target "ConicSection")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 139 28) (end 139 39)) (probe (position 139 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 139 47) (end 139 63)) (probe (position 139 47))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 138 28) (end 138 39)) (probe (position 138 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 138 47) (end 138 63)) (probe (position 138 47))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 326 25) (end 326 37)) (probe (position 326 25))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid"))) (kind specialization) (ordinal 0) (authored-target "ConicSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 333 28) (end 333 39)) (probe (position 333 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 333 47) (end 333 63)) (probe (position 333 47))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 332 29) (end 332 40)) (probe (position 332 29))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 332 48) (end 332 64)) (probe (position 332 48))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 48 18) (end 48 29)) (probe (position 48 18))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Line"))) (kind specialization) (ordinal 0) (authored-target "PlanarCurve")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 54 16) (end 54 22)) (probe (position 54 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 55 16) (end 55 35)) (probe (position 55 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 121 22) (end 121 34)) (probe (position 121 22))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola"))) (kind specialization) (ordinal 0) (authored-target "ConicSection")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSection")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 127 28) (end 127 39)) (probe (position 127 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 127 47) (end 127 63)) (probe (position 127 47))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 315 24) (end 315 36)) (probe (position 315 24))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid"))) (kind specialization) (ordinal 0) (authored-target "ConicSurface")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::ConicSurface")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 321 28) (end 321 39)) (probe (position 321 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 321 47) (end 321 63)) (probe (position 321 47))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 58 27) (end 58 65)) (probe (position 58 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path"))) (kind specialization) (ordinal 0) (authored-target "StructuredSpaceObject::StructuredCurve")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 24 25) (end 24 30)) (probe (position 24 25))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve"))) (kind specialization) (ordinal 0) (authored-target "Curve")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 30 16) (end 30 22)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 32 16) (end 32 35)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 36 27) (end 36 34)) (probe (position 36 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarSurface"))) (kind specialization) (ordinal 0) (authored-target "Surface")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 42 16) (end 42 20)) (probe (position 42 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "area")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 43 16) (end 43 35)) (probe (position 43 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 142 21) (end 142 25)) (probe (position 142 21))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 0) (authored-target "Path")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Path")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 142 27) (end 142 38)) (probe (position 142 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 1) (authored-target "PlanarCurve")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::PlanarCurve")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 150 16) (end 150 24)) (probe (position 150 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isClosed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 540 24) (end 540 29)) (probe (position 540 24))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron"))) (kind specialization) (ordinal 0) (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 546 16) (end 546 24)) (probe (position 546 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isClosed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 556 16) (end 556 35)) (probe (position 556 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 558 16) (end 558 21)) (probe (position 558 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "genus")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 822 21) (end 822 31)) (probe (position 822 21))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid"))) (kind specialization) (ordinal 0) (authored-target "Polyhedron")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polyhedron")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 830 16) (end 830 22)) (probe (position 830 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "height")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 831 16) (end 831 23)) (probe (position 831 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 832 16) (end 832 23)) (probe (position 832 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 840 25) (end 840 33)) (probe (position 840 25))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind featureTyping) (ordinal 0) (authored-target "Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 195 27) (end 195 34)) (probe (position 195 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral"))) (kind specialization) (ordinal 0) (authored-target "Polygon")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 214 23) (end 214 36)) (probe (position 214 23))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Rectangle"))) (kind specialization) (ordinal 0) (authored-target "Quadrilateral")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Quadrilateral")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 220 16) (end 220 22)) (probe (position 220 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 221 16) (end 221 21)) (probe (position 221 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 797 31) (end 797 37)) (probe (position 797 31))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularCuboid"))) (kind specialization) (ordinal 0) (authored-target "Cuboid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Cuboid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 803 16) (end 803 22)) (probe (position 803 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 804 16) (end 804 21)) (probe (position 804 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 805 16) (end 805 22)) (probe (position 805 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "height")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 882 32) (end 882 39)) (probe (position 882 32))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularPyramid"))) (kind specialization) (ordinal 0) (authored-target "Pyramid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 888 16) (end 888 26)) (probe (position 888 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "baseLength")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 889 16) (end 889 25)) (probe (position 889 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseWidth")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 367 31) (end 367 37)) (probe (position 367 31))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid"))) (kind specialization) (ordinal 0) (authored-target "Toroid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 373 30) (end 373 41)) (probe (position 373 30))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 373 49) (end 373 65)) (probe (position 373 49))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 374 30) (end 374 41)) (probe (position 374 30))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 374 49) (end 374 65)) (probe (position 374 49))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 478 31) (end 478 43)) (probe (position 478 31))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCone"))) (kind specialization) (ordinal 0) (authored-target "CircularCone")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCone")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 484 16) (end 484 23)) (probe (position 484 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 485 16) (end 485 23)) (probe (position 485 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 484 40) (end 484 43)) (probe (position 484 40))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 485 40) (end 485 43)) (probe (position 485 40))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 530 35) (end 530 51)) (probe (position 530 35))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind specialization) (ordinal 0) (authored-target "CircularCylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CircularCylinder")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 536 16) (end 536 23)) (probe (position 536 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 537 16) (end 537 23)) (probe (position 537 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "yoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 536 40) (end 536 43)) (probe (position 536 40))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 537 40) (end 537 43)) (probe (position 537 40))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 180 27) (end 180 35)) (probe (position 180 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangle"))) (kind specialization) (ordinal 0) (authored-target "Triangle")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 186 16) (end 186 23)) (probe (position 186 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 709 34) (end 709 49)) (probe (position 709 34))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind specialization) (ordinal 0) (authored-target "TriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 716 16) (end 716 22)) (probe (position 716 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 717 16) (end 717 21)) (probe (position 717 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 718 16) (end 718 22)) (probe (position 718 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "height")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 229 28) (end 229 68)) (probe (position 229 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell"))) (kind specialization) (ordinal 0) (authored-target "StructuredSpaceObject::StructuredSurface")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 303 20) (end 303 29)) (probe (position 303 20))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Sphere"))) (kind specialization) (ordinal 0) (authored-target "Ellipsoid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 309 16) (end 309 22)) (probe (position 309 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 310 16) (end 310 25)) (probe (position 310 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "semiAxis1")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis1")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 311 16) (end 311 25)) (probe (position 311 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "semiAxis2")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis2")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 312 16) (end 312 25)) (probe (position 312 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "semiAxis3")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Ellipsoid::semiAxis3")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 865 25) (end 865 32)) (probe (position 865 25))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Tetrahedron"))) (kind specialization) (ordinal 0) (authored-target "Pyramid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Pyramid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 871 16) (end 871 26)) (probe (position 871 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "baseLength")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 872 16) (end 872 25)) (probe (position 872 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseWidth")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 336 20) (end 336 25)) (probe (position 336 20))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid"))) (kind specialization) (ordinal 0) (authored-target "Shell")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Shell")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 351 16) (end 351 21)) (probe (position 351 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "genus")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 343 31) (end 343 42)) (probe (position 343 31))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 343 50) (end 343 66)) (probe (position 343 50))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 354 19) (end 354 25)) (probe (position 354 19))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus"))) (kind specialization) (ordinal 0) (authored-target "Toroid")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 360 28) (end 360 44)) (probe (position 360 28))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind redefinition) (ordinal 0) (authored-target "revolutionRadius")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Toroid::revolutionRadius")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 361 26) (end 361 37)) (probe (position 361 26))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 361 45) (end 361 61)) (probe (position 361 45))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 158 22) (end 158 29)) (probe (position 158 22))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Triangle"))) (kind specialization) (ordinal 0) (authored-target "Polygon")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Polygon")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 165 16) (end 165 22)) (probe (position 165 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 166 16) (end 166 21)) (probe (position 166 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 167 16) (end 167 23)) (probe (position 167 16))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "xoffset")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 679 29) (end 679 52)) (probe (position 679 29))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::TriangularPrism"))) (kind specialization) (ordinal 0) (authored-target "CuboidOrTriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 743 17) (end 743 37)) (probe (position 743 17))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::Wedge"))) (kind aliasBinding) (ordinal 0) (authored-target "RightTriangularPrism")
      (outcome (status resolved) (target (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::RightTriangularPrism")))))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 79 24) (end 79 35)) (probe (position 79 24))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseLength"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 80 23) (end 80 34)) (probe (position 80 23))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::baseWidth"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 75 27) (end 75 38)) (probe (position 75 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMajorAxis"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 76 27) (end 76 38)) (probe (position 76 27))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::semiMinorAxis"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 77 21) (end 77 32)) (probe (position 77 21))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::xoffset"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/shape_items.md") (range (start 78 21) (end 78 32)) (probe (position 78 21))
    (reference (id (source (node (document "memory://snapshot/shape_items.md") (qualified-name "ShapeItems::yoffset"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
)
~~~
