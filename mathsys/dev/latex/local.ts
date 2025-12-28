//^
//^ MAPPINGS
//^

//> MAPPINGS -> VARIABLES
export const VARIABLES = {
    //~ VARIABLES -> 7 GREEK
    "epsilon": String.raw`\epsilon `,
    "Epsilon": String.raw`E`,
    "omicron": String.raw`\omicron `,
    "Omicron": String.raw`O`,
    "upsilon": String.raw`\upsilon `,
    "Upsilon": String.raw`\Upsilon `,
    //~ VARIABLES -> 6 GREEK
    "lambda": String.raw`\lambda `,
    "Lambda": String.raw`\Lambda `,
    //~ VARIABLES -> 5 GREEK
    "alpha": String.raw`\alpha `,
    "Alpha": String.raw`A`,
    "gamma": String.raw`\gamma `,
    "Gamma": String.raw`\Gamma `,
    "delta": String.raw`\delta `,
    "Delta": String.raw`\Delta `,
    "theta": String.raw`\theta `,
    "Theta": String.raw`\Theta `,
    "kappa": String.raw`\kappa `,
    "Kappa": String.raw`K`,
    "sigma": String.raw`\sigma `,
    "Sigma": String.raw`\Sigma `,
    "omega": String.raw`\omega `,
    "Omega": String.raw`\Omega `,
    //~ VARIABLES -> 5 DOTLESS
    "imath": String.raw`\imath `,
    "jmath": String.raw`\jmath `,
    //~ VARIABLES -> 4 GREEK
    "beta": String.raw`\beta `,
    "Beta": String.raw`B`,
    "zeta": String.raw`\zeta `,
    "Zeta": String.raw`Z`,
    "iota": String.raw`\iota `,
    "Iota": String.raw`I`,
    //~ VARIABLES -> 4 EXTRA
    "hbar": String.raw`\hbar `,
    //~ VARIABLES -> 3 GREEK
    "eta": String.raw`\eta `,
    "Eta": String.raw`H`,
    "rho": String.raw`\rho `,
    "Rho": String.raw`P`,
    "tau": String.raw`\tau `,
    "Tau": String.raw`T`,
    "phi": String.raw`\phi `,
    "Phi": String.raw`\Phi `,
    "chi": String.raw`\chi `,
    "Chi": String.raw`X`,
    "psi": String.raw`\psi `,
    "Psi": String.raw`\Psi `,
    //~ VARIABLES -> 3 EXTRA
    "ell": String.raw`\ell `,
    //~ VARIABLES -> 2 GREEK
    "mu": String.raw`\mu `,
    "Mu": String.raw`M`,
    "nu": String.raw`\nu `,
    "Nu": String.raw`N`,
    "xi": String.raw`\xi `,
    "Xi": String.raw`\Xi `,
    "pi": String.raw`\pi `,
    "Pi": String.raw`\Pi `,
    //~ VARIABLES -> WEIERSTRASS
    "wp": String.raw`\wp `
}

//> MAPPINGS -> SPECIAL
export const SPECIAL: Record<string, string> = {
    '\\': String.raw`\\`,
    '{': String.raw`\{`,
    '}': String.raw`\}`,
    '$': String.raw`\$`
}

//> MAPPINGS -> CONVERSION TABLE
export const CONVERSION: Record<string, (string: string) => string> = {
    null: (name: string) => name,
    "@Infinite": (name: string) => String.raw`\overset{\infty}{${name}}`,
    "@Integer": (name: string) => name,
    "@Natural": (name: string) => name,
    "@Nexists": (name: string) => String.raw`\color{gray}${name}\color{black}`,
    "@Rational": (name: string) => name,
    "@Tensor": (name: string) => String.raw`\overline{${name}}`,
    "@Undefined": (name: string) => String.raw`\overset{?}{${name}}`,
    "@Variable": (name: string) => String.raw`{^{*}${name}}`,
    "@Whole": (name: string) => name
}


//^
//^ TYPES
//^

//> TYPES -> REGISTER
export const types: Record<string, null | string> = {}