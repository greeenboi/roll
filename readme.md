# Roll 🎲

<i style="color:teal">Just Roll with it.</i>

A colorful implementation of curl's http api.


> Sample command

```bash
$ cargo run https://www.boredapi.com/api/activity 
```

> Response

```json
{
    "activity":"Go to the gym",
    "type":"recreational",
    "participants":1,
    "price":0.2,
    "link":"",
    "key":"4387026",
    "accessibility":0.1
}
```
***

# Usage and flags

> verbose flag

```bash
$ cargo run -v https://www.boredapi.com/api/activity 
```

```bash
$ cargo run --verbose https://www.boredapi.com/api/activity 
```

<br/><br/>

> headers flag

```bash
$ cargo run https://www.boredapi.com/api/activity -H 'your headers here'
```

<br/><br/>

> method flag

```bash
$ cargo run https://www.boredapi.com/api/activity -X 'your Method'
```

<br/><br/>

> data flag

```bash
$ cargo run https://www.boredapi.com/api/activity -d 'your data / Payload'
```

<br/><br/>

***
[Find more APis Here](https://apipheny.io/free-api/)

Made with Rust. 🦀
