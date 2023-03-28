# Desk Programming Language Formal Definition

## Syntax
$$
\begin{aligned}
t ::=\ & \\& t_{ctx} \ t_{ty}\ t_{arg1},\ldots \\
\mid\ & \Pi t_{ctx} \ t_{fam}\ (t_{fam}\ \text{has at least one}\ \\&\ t_{ctx}\ \text{t)} \\
\mid\ & \Sigma t_{ctx} \ t_{fam}\ (t_{fam}\ \text{has at least one}\ \\&\ t_{ctx}\ \text{t)} \\
\mid\ & \\{ t_1,\ldots \\} \\
\mid\ & t_{term}:\ t_{ty} \\
\mid\ & \text{let}\ t_{def}\ \text{in}\ t_e \\
\mid\ & \text{letrec}\ t_{ident}\ =\ t_{def}\ \text{in}\ t_e \\
\mid\ & \text{branch}\ t\ \text{begin}\ t_{ty1} \rightarrow t_{case1},\ldots\ \text{end} \\
\mid\ & \text{!} \ t_i \sim> t_o \\
\mid\ & \text{handle}\ t\ \text{begin}\ t_{i1} \sim> t_{o1} \rightarrow t_{h1},\ldots\ \text{end} \\
\mid\ & \text{@}\ t\ \mid\ \text{@@}\ t \\
\mid\ & \\# t_{attr}\ t
\\
program ::=\ & \cdot \mid\ program\ t_{alias} = t_{of} \mid\ program\ t \\
\\
\Gamma ::=\ & \cdot \mid\ \Gamma\ t
\\
{\huge \varepsilon} ::=\ & \cdot \mid\ {\huge \varepsilon}\ t \\
\end{aligned}
$$
