create table links (
  id serial primary key,
  original_link text not null,
  short_link text not null,
  created_at timestamp not null default now()
);