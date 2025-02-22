## Useful commands

EG_SIMULATOR_DUMP=screenshots/screenshot.png cargo +nightly run

## Progression

- positionnement des coins : OK
- cas ou la bordure est plus épaisse que le rectangle : OK
- propreté du code pour les coins : OK

- calculer le nombre de cercles à dessiner en fonction de `border_size` : OK
- les positioner de façon symétrique au centre : ACK (inutile)
- rendre le code plus propre (notamment pour le round !) : 

- extraire le code des coins dans une fonction à part du trait `ExtendedStyledDrawable`
et expliquer le principe
- s'informer sur les tests dans embedded-graphics :D