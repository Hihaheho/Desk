# Desk Programming Language Formal Definition

!Note: not an easy-understanding guidance

## Syntax

$$
\begin{aligned}
t & ::=\ n \mid i \mid r \mid f \mid c \\
& \mid \Pi\ t_{fam}\ \text{where}\ t_{pred} \\
& \mid \Sigma\ t_{fam}\ \text{where}\ t_{pred} \\
& \mid \lambda\ t_{par} \rightarrow t_{ret} \\
& \mid [t_1,\ldots] \\
& \mid [t;n] \\
& \mid \langle t_{k1} \Rightarrow t_{v1},\ldots\rangle \\
& \mid \text{let}\ t_{def}\ \text{in}\ t_e \\
& \mid \And\ t_{ty}\ t_{arg1},\ldots \\
& \mid \text{branch}\ t\ \text{begin}\ t_{ty1} \Rightarrow t_{case1},\ldots\ \text{end} \\
& \mid \text{!} \ t_i \sim> t_o \\
& \mid \text{handle}\ t\ \text{begin}\ t_{i1} \sim> t_{o1} \Rightarrow t_{h1},\ldots\ \text{end} \\
& \mid \text{@}\ t\ \\
& \mid t_{term}\ \text{is}\ t_{ty} \\
& \mid \text{?} \\
& \mid \sharp\ \text{dson}\ t \\
\\
program & ::=\ \cdot \mid program\ t_{alias} = t_{of} \mid program\ t \\
\\
\Gamma & ::=\ \cdot \mid \Gamma\ t \\
\end{aligned}$$
