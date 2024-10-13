# Algorithme détaillé de gestion du trafic et des feux

## 1. Initialisation

1.1. Créer quatre voies (lanes) correspondant aux quatre directions de l'intersection.
1.2. Initialiser les feux de circulation pour chaque voie (rouge par défaut).
1.3. Définir les points d'arrêt et les points de changement de direction pour chaque voie.

## 2. Boucle principale

Pour chaque frame :

2.1. Gérer les entrées utilisateur (ajout de nouveaux véhicules).
2.2. Mettre à jour chaque voie.
2.3. Mettre à jour les feux de circulation.
2.4. Dessiner la carte et tous les éléments.

## 3. Gestion des véhicules

Pour chaque voie :

3.1. Ajouter de nouveaux véhicules si nécessaire.
3.2. Pour chaque véhicule :
    a. Déterminer s'il peut avancer :
       - Si le feu est rouge et le véhicule est au point d'arrêt, stopper.
       - Si la distance avec le véhicule précédent est inférieure à la distance de sécurité, stopper.
    b. Si le véhicule peut avancer :
       - Mettre à jour sa position.
       - Vérifier s'il doit changer de direction (basé sur sa couleur et sa destination).
    c. Si le véhicule a atteint sa destination, le retirer de la voie.

## 4. Gestion des priorités et des feux

4.1. Vérifier si une voie est actuellement en phase de traversée (Crossing) :
    - Si oui, ne rien changer.
    - Si non, passer à l'étape 4.2.

4.2. Pour chaque voie :
    a. Identifier les véhicules en attente (stage == Waiting).
    b. Calculer la distance du véhicule le plus proche du point d'arrêt.
    c. Compter le nombre de véhicules en attente.

4.3. Sélectionner la voie prioritaire :
    a. Choisir la voie avec le véhicule le plus proche du point d'arrêt.
    b. En cas d'égalité, choisir la voie avec le plus grand nombre de véhicules en attente.

4.4. Pour la voie prioritaire :
    a. Changer le feu au vert.
    b. Mettre le stage de la voie à Crossing.

4.5. Pour les autres voies :
    a. Maintenir ou changer le feu au rouge.
    b. Mettre le stage de la voie à Waiting.

## 5. Détails de l'algorithme de changement de feu (fonction update_traffic_lights)

5.1. Vérifier si une voie est en phase de traversée (Crossing) :
    - Si oui, sortir de la fonction sans rien changer.

5.2. Initialiser les variables pour trouver la prochaine voie à traverser :
    - next_cross_lane : None
    - min_distance : infini
    - max_vehicle_count : 0

5.3. Pour chaque voie :
    a. Filtrer les véhicules en attente.
    b. Si des véhicules sont en attente :
       - Obtenir la distance du véhicule le plus proche.
       - Comparer cette distance avec min_distance.
       - Si la distance est plus petite ou égale et qu'il y a plus de véhicules en attente :
         * Mettre à jour min_distance, max_vehicle_count et next_cross_lane.

5.4. Si une voie prioritaire a été trouvée (next_cross_lane != None) :
    - Changer son feu au vert.
    - Mettre son stage à Crossing.

## 6. Gestion des véhicules dans une voie (méthode Lane::update)

6.1. Dessiner le feu de circulation.
6.2. Arrêter les véhicules si nécessaire.
6.3. Gérer la traversée de l'intersection.
6.4. Pour chaque véhicule :
    a. Mettre à jour sa position.
    b. Le supprimer s'il a atteint la fin de la voie.

## 7. Gestion de la traversée (méthode Lane::cross)

7.1. Si le stage de la voie n'est pas Crossing, sortir de la méthode.
7.2. Identifier les véhicules qui ont déjà traversé le point d'arrêt.
7.3. Si aucun véhicule n'a traversé le point d'arrêt :
    a. Vérifier la distance du premier véhicule au point d'arrêt.
    b. Si cette distance est supérieure à deux fois la distance de sécurité :
       - Changer le feu au rouge.
       - Mettre le stage de la voie à Waiting.
    c. Sinon, ne rien faire (continuer la traversée).