create table sensors_data (
	id text not null,
	protocol text not null,
	dt_start timestamp not null,
	dt_end timestamp,
	temperature real,
	humidity real,
	Primary key (id,protocol,dt_start)
);
