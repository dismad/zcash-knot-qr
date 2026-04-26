https://github.com/3-manifolds/sage_appimage/releases

https://www.rolandvdv.nl/Theta/






# === YOUR ZCASH UA (213 chars) ===
ua = "u1ckeydud0996ftppqrnpdsqyeq4e57qcyjr4raht4dc8j3njuyj3gmm9yk7hq9k88cdkqfuqusgpcpjfhwu3plm2vrd32g8du78kzkm5un357r4vkhz4vhxd4yfl8zvszk99cmsc89qv4trd7jzkcs8h6lukzgy25j8cv76p0g603nrrg6yt6cxsh2v8rmkasskd69ylfyphhjyv0cxs"

load("Theta.sage")
load("knots.sage")

# Pad to exactly 217 characters (matches 14a479)
padding = "===="
ua_padded = ua + padding
print("Padded UA length:", len(ua_padded))

# Load base knot
theta_base = Theta(Knot['14a479'])

# Get numerator and denominator
num = theta_base.numerator()
den = theta_base.denominator()

print("Base knot has", num.number_of_terms(), "nonzero terms")

# Get monomials and sort canonically
monomials = num.monomials()
sorted_monomials = sorted(monomials, 
    key=lambda m: (m.total_degree(), 
                   m.exponents()[0][0],   # x exponent
                   m.exponents()[0][1]))  # y exponent

R = num.parent()

# Build new coefficients from UA characters
new_coeffs = {}
for i, mon in enumerate(sorted_monomials):
    char = ua_padded[i]
    coeff = ord(char) - 128          # signed range for nice coloring
    new_coeffs[mon] = coeff

# Build new polynomial
new_poly = sum(coeff * mon for mon, coeff in new_coeffs.items())

# Create the final θ (same denominator as original)
new_theta = new_poly / den

print("🎉 SUCCESS! New θ created with", new_poly.number_of_terms(), "terms")
print("θ:", new_theta)
print("Denominator:", den)