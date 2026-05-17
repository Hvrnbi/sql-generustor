# sql-generustor

## 🥖

Une application de terminal simple et légère écrite en rust pour générer des commandes d'insertion SQL.

Entièrement compatible avec SQLite, et partiellement avec MySQL.

### Installation

Le moyen le plus simple d'installer cette application est d'utiliser cargo :

```cargo install sql-generustor```

### Utilisation

Cette application prend 4 arguments : la table sql, les colonnes, les types des colonnes, et le nombre de lignes à générer.

Exemple d'utilisation :

```
$ sql-generustor gens nom,prénom,taille,mdp text,text,real,text 4
insert into gens (nom,prénom,taille,mdp) values ('Hoxha','Achille',0.14867234,'MyNameIsNobody');
insert into gens (nom,prénom,taille,mdp) values ('Haji','Frank',0.92468286,'ÊtreOuNÊtrePasTelleEstLaQuestion');
insert into gens (nom,prénom,taille,mdp) values ('Lake','Michael',0.37070015,'LesChaussettesDeLArchiduchesseSontEllesSechesArchiSeches');
insert into gens (nom,prénom,taille,mdp) values ('Meredith','Mallory',0.0016351843,'JaimeLesPates123');
```

### Fonctionnalités

Des données cohérentes sont générées si vos colonnes ont des noms spécifiques (indifféremment de la présence d'accents ou de majuscules) :
- nom, name, lastname, last name, last-name, last_name -> nom de famille
- prenom, firstname, first name, first-name, first_name -> prénom
- motdepasse, mot de passe, mot-de-passe, mot_de_passe, mdp, password, passwd -> mot de passe
- age -> âge
- annee, an, year -> année


## 🍵

A simple and lightweight command-line application written in Rust to generate SQL insertion commands.

Fully compatible with SQLite and partially with MySQL.

### Installation

The easiest way to install this app is using Cargo:

```cargo install sql-generustor```

### Usage

This application takes 4 arguments: the SQL table, the columns, their types, and the number of rows to generate.

Usage example:

```
$ sql-generustor people name,firstname,height,password text,text,real,text 4
insert into people (name,firstname,height,password) values ('Hoxha','Achille',0.14867234,'MyNameIsNobody');
insert into people (name,firstname,height,password) values ('Haji','Frank',0.92468286,'ÊtreOuNÊtrePasTelleEstLaQuestion');
insert into people (name,firstname,height,password) values ('Lake','Michael',0.37070015,'LesChaussettesDeLArchiduchesseSontEllesSechesArchiSeches');
insert into people (name,firstname,height,password) values ('Meredith','Mallory',0.0016351843,'JaimeLesPates123');
```

### Features

Consistent data is generated if your columns have specific names (regardless of accents or capitalization):
- nom, name, lastname, last name, last-name, last_name -> last name
- prenom, firstname, first name, first-name, first_name -> first name
- motdepasse, mot de passe, mot-de-passe, mot_de_passe, mdp, password, passwd -> password
- age -> age
- annee, an, year -> year


