# db-evolution
A db evolution tool in rust. It's for versioning DB changes. It connects to a remote db and run sqls in a given folder. Each sql file will be a changelog and assigned to a checksum once it's finished.

Currently it only support cockroachdb.

## Evolutions scripts
Evolution tracks your database evolutions using several evolutions script. These scripts are written in plain old SQL and should be located in the db/evolutions directory of your application.

The first script is named 1.sql, the second script 2.sql, and so on…

Each script describes the required transformations.
For example, take a look at this first evolution script that bootstrap a basic application:

\# Users schema
 
CREATE TABLE User (
    id bigint(20) NOT NULL AUTO_INCREMENT,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    fullname varchar(255) NOT NULL,
    isAdmin boolean NOT NULL,
    PRIMARY KEY (id)
);
 
## Inconsistent states

Sometimes you will make a mistake in your evolution scripts, and they will fail. In this case, Play will mark your database schema as being in an inconsistent state and will ask you to manually resolve the problem before continuing.

For example, the script of this evolution has an error:

\# Add another column to User

ALTER TABLE Userxxx ADD company varchar(255);
 
All changes are recorded in migtation.evoultions table. The failure reason is also recorded in.



