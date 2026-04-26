# zcash-knot-qr

![ua](https://github.com/user-attachments/assets/3209bca9-3023-441e-ae2d-8fd0600387be)

This project turns a long Zcash Unified Address (the `u1...` string) into a beautiful, unique hexagonal "knot QR code" that looks exactly like the visualizations in the
following Quanta Magazine article:

https://www.quantamagazine.org/a-powerful-new-qr-code-untangles-maths-knottiest-knots-20260422/


# Install

```bash

git clone https://github.com/dismad/zcash-knot-qr.git
cd zcash-knot-qr
cargo build -r
./target/release/zcash-knot-qr "your ua here"
```

# Background
## 1. The Knot Diagram (Geometric Picture)

![knot](https://github.com/user-attachments/assets/18199097-afaf-4e7b-a57c-7b8e7e6a0a6f)

the **standard planar projection of knot 14a479**.

- It is a closed loop with **exactly 14 crossings**.
- Each crossing has a sign (+ or –) based on its orientation.
- This is the visual, topological picture of knot 14a479 .

## 2. The Lattice (Algebraic Fingerprint)

![lattice](https://github.com/user-attachments/assets/46be930a-ec76-4fc2-a331-f814ec11fd16)


The colorful hexagonal plot is **not** a drawing of the knot. It is an **algebraic/combinatorial encoding** produced by the Bar-Natan–van der Veen Θ-invariant.

- For knot 14a479, the Θ-invariant computes a two-variable Laurent polynomial **θ**.
- This polynomial has **exactly 217 nonzero terms** (monomials \(x^a y^b\) with integer coefficients).
- Each term lives at a lattice point \((a, b)\) in the \((t_1, t_2)\) plane.
- These 217 points form the **machine-readable skeleton** the computer uses to compute θ.

## 3. How the Knot Diagram and the Lattice Are Related
They describe the **same knot** in two completely different languages.

The connection is **algebraic**, not geometric:

- From the knot diagram, the algorithm extracts a **planar diagram (PD) code** or Gauss code.
- For each of the 14 crossings, it creates one or more lattice points.
- The final 217 lattice points are the result of matrix construction, grading, and inversion inside the Θ formula.

Each of the 14 signed lattice points corresponds directly to one specific crossing in the diagram (in the algorithm’s ordering):

| Sign | Lattice point | Corresponds to crossing |
|------|---------------|-------------------------|
| +1   | (0,3)         | #1                      |
| +1   | (2,7)         | #2                      |
| –1   | (4,11)        | #3                      |
| +1   | (6,1)         | #4                      |
| –1   | (8,13)        | #5                      |
| +1   | (10,23)       | #6                      |
| –1   | (12,5)        | #7                      |
| –1   | (14,19)       | #8                      |
| –1   | (16,27)       | #9                      |
| +1   | (18,25)       | #10                     |
| –1   | (20,15)       | #11                     |
| +1   | (22,17)       | #12                     |
| +1   | (24,9)        | #13                     |
| +1   | (26,21)       | #14                     |

The \((x, y)\) coordinates are **abstract integer labels** coming from the grading of arcs, not physical positions on the blue diagram. The sign comes from the crossing orientation.

### What the 14 points represent
They are the input to the algorithm:

Each point is tied to one specific crossing in the blue knot diagram.
The sign (+ or –) comes from the crossing orientation.
The coordinates $  (a, b)  $ are abstract labels derived from the Gauss code / PD code ordering.

### What the 217 points represent
When the Θ-invariant algorithm runs, it does a lot of heavy algebraic work:

Builds a large symbolic matrix indexed by arcs and crossings.
Computes a graded inverse (Green function).
Sums multiple contributions (single-crossing terms, pairwise terms, etc.).
The final result is the full θ polynomial, which for 14a479 expands to exactly 217 nonzero monomials.

**Summary**:  
- Blue diagram = geometric picture of the knot (what it looks like).  
- Lattice = algebraic data structure with one signed point per crossing (what the computer uses to compute Θ).

## 4. The Original Math (Quanta Article)
Mathematicians invented a powerful new knot invariant called **θ** (part of the Θ-invariant by Bar-Natan–van der Veen).

- For any knot they compute a complicated two-variable Laurent polynomial θ.
- For 14a479 it has **217 nonzero terms**.
- They plot the coefficients on a 2D lattice.
- They apply the **shear transformation** \( y \leftarrow y + \frac{a}{2} \).
- They render the points as colored hexagons (red = positive, blue = negative, intensity = magnitude).
- Result: a beautiful hexagonal “QR code” that uniquely fingerprints the knot.

## 5. Your Idea (The Creative Twist)
You keep **exactly the same mathematical canvas** (the fixed 217-point lattice of 14a479) but change what the colors represent.

- **Input**: a Zcash Unified Address (the long `u1...` string).
- **Mapping**: each character in the padded UA (exactly 217 characters) is turned into a coefficient on the lattice (using the prime-based rule: prime ASCII value → positive/red, non-prime → negative/blue).
- **Output**: the same style of colorful hexagonal picture, but now it encodes **your address** instead of a knot.

The knot diagram provides the geometric origin; the lattice provides the fixed algebraic template; your UA provides the data painted onto that template.

## 6. How Rust Brings It All Together
The single Rust binary does the entire pipeline in one fast, standalone executable:

1. Takes your UA → pads it to exactly 217 characters.
2. Maps each character to a coefficient (prime-based red/blue rule).
3. Places the 217 coefficients onto the exact lattice points of 14a479.
4. Applies the article’s transformations:
   - Shear \( y = b + a/2 \)
   - Centering + rotation (to make the shape more symmetric, like Figure 4)
   - Renders small hexagons with red/blue coloring and black borders.
5. Outputs a clean PNG that looks like the Quanta article’s knot QR codes — except the colors now come from your Zcash address.

## The Complete Loop
**Knot diagram** (geometric picture)  
→ **Θ-invariant algorithm** (matrix construction + grading)  
→ **217-point algebraic lattice** (machine-readable skeleton)  
→ **UA character mapping** (your data painted onto the lattice)  
→ **Rust transformations** (shear + centering + rotation + hexagon rendering)  
→ **Final symmetric hexagonal QR code** (your unique visual fingerprint)


