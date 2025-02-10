# most used mechanics

```sql
select link.value as mechanic, count(value) as number from link
join item_link on link.id = item_link.link_id
where link_type = 'boardgamemechanic'
group by value
order by number desc
```
