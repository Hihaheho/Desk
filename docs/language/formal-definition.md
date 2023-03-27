# Desk Programming Language Formal Definition

!Note: not an easy-understanding guidance

## Syntax
$$
\begin{aligned}
n &\in \mathbb{N}\\
i &\in \mathbb{Z}\\
r &\in \mathbb{R}\\
c &\in \text{Alphabets}
\end{aligned}
$$

$$
\begin{aligned}
t & ::=\ n \mid i \mid i / n \mid r \mid c \\
& \mid \Pi t_{ctx} \ t_{fam}\\
& \mid \Sigma t_{ctx} \ t_{fam}\\
& \mid \\{ t_1,\ldots \\} \\
& \mid [ t_1,\ldots ] \\
& \mid \\{ t_{k1} \Rightarrow t_{v1},\ldots \\} \\
& \mid \text{let}\ t_{def}\ \text{in}\ t_e \\
& \mid \text{letrec}\ t_{ident}\ =\ t_{def}\ \text{in}\ t_e \\
& \mid \\& t_{ctx} \ t_{ty}\ t_{arg1},\ldots \\
& \mid \text{branch}\ t\ \text{begin}\ t_{ty1} \Rightarrow t_{case1},\ldots\ \text{end} \\
& \mid \text{!} \ t_i \sim> t_o \\
& \mid \text{handle}\ t\ \text{begin}\ t_{i1} \sim> t_{o1} \Rightarrow t_{h1},\ldots\ \text{end} \\
& \mid \text{@}\ t\ \\
& \mid t_{term}\ \text{is}\ t_{ty} \\
& \mid \text{?} \\
& \mid \sharp\ \text{DSON}\ t \\
\\
program & ::=\ \cdot \mid program\ t_{alias} = t_{of} \mid program\ t \\
\\
\Gamma & ::=\ \cdot \mid \Gamma\ t \\
\\
{\huge \varepsilon} & ::=\ \cdot \mid {\huge \varepsilon}\ t \\
\end{aligned}$$
