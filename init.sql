drop table if exists estudiante;

create table estudiante (
    id int primary key,
    nombre varchar(150),
    telefono varchar(12),
    fecha_nacimiento varchar(10)
);

insert into estudiante values (100, 'Andres Estrella', '829-638-5829', '27/02/2000')