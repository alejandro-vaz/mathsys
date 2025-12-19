#^
#^  MAPPINGS
#^

#> MAPPINGS -> VARIABLES
VARIABLES = {
    "epsilon": r"\epsilon ",
    "Epsilon": r"E",
    "omicron": r"\omicron ",
    "Omicron": r"O",
    "upsilon": r"\upsilon ",
    "Upsilon": r"\Upsilon ",
    "lambda": r"\lambda ",
    "Lambda": r"\Lambda ",
    "alpha": r"\alpha ",
    "Alpha": r"A",
    "gamma": r"\gamma ",
    "Gamma": r"\Gamma ",
    "delta": r"\delta ",
    "Delta": r"\Delta ",
    "theta": r"\theta ",
    "Theta": r"\Theta ",
    "kappa": r"\kappa ",
    "Kappa": r"K",
    "sigma": r"\sigma ",
    "Sigma": r"\Sigma ",
    "omega": r"\omega ",
    "Omega": r"\Omega ",
    "beta": r"\beta ",
    "Beta": r"B",
    "zeta": r"\zeta ",
    "Zeta": r"Z",
    "iota": r"\iota ",
    "Iota": r"I",
    "eta": r"\eta ",
    "Eta": r"H",
    "rho": r"\rho ",
    "Rho": r"P",
    "tau": r"\tau ",
    "Tau": r"T",
    "phi": r"\phi ",
    "Phi": r"\Phi ",
    "chi": r"\chi ",
    "Chi": r"X",
    "psi": r"\psi ",
    "Psi": r"\Psi ",
    "mu": r"\mu ",
    "Mu": r"M",
    "nu": r"\nu ",
    "Nu": r"N",
    "xi": r"\xi ",
    "Xi": r"\Xi ",
    "pi": r"\pi ",
    "Pi": r"\pi "
}

#> MAPPINGS -> SPECIAL
SPECIAL = {
    '\\': r'\\',
    '{': r'\{',
    '}': r'\}',
    '$': r'\$'
}

#> MAPPINGS -> CONVERSION TABLE
CONVERSION = {
    None: lambda name: name,
    "Any": lambda name: name,
    "Infinite": lambda name: f"\overset{{\infty}}{{{name}}}",
    "Nexists": lambda name: fr"\color{{gray}}{name}\color{{black}}",
    "Number": lambda name: name,
    "Tensor": lambda name: f"\overline{{{name}}}",
    "Undefined": lambda name: f"\overset{{?}}{{{name}}}",
    "Variable": lambda name: f"{{^{{*}}{name}}}"
}


#^
#^  TYPES 
#^

#> TYPES -> REGISTER
types = {}