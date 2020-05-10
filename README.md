# Yew-template

My setup for web devlopment with [Yew](https://github.com/yewstack/yew).

Based on [yew-wasm-pack-template](https://github.com/yewstack/yew-wasm-pack-template).

# How to use

Make sure you installed `rust`, `wasm-pack`, and `npm` or `yarn`.
(I recommend to use `yarn` for its speed)

## 1. Install dependency
```
yarn install
```

## 2. Development
Start development server on `localhost:8080`. It watches source file changes and rebuild automatically.
```
yarn dev
```

## 3. Production build
Make production build to `dist` folder.
```
yarn build
```


# Selecting Optimization Options

To reduce wasm file size, There are [some optimization options](https://rustwasm.github.io/docs/book/reference/code-size.html).

I wanted to know its effects, so I took benchmark. Here is the results.

(I didn't check its speed, size only. Used `rustc 1.43.0`.)

||Production Build|Development Build|
| - | - | - |
|No Options|84.5 KiB|750 KiB|
|LTO|86.4 KiB|763 KiB|
|opt-level = s|76.8 KiB|131 KiB|
|opt-level = z|81.3 KiB|157 KiB|
|LTO + opt-level = s|76.4 KiB|129 KiB|
|LTO + opr-level = z|81.3 Kib|156 KiB|


## Graph
**"No Options" and "LTO" on development build isn't on here because they're too big and prevents see differences.**

![Graph Image](https://imgur.com/RNpF4tw.png)

So this template is using "LTO + opt-level = s" which is smallest output size on production build.

If you want to change this option, see cargo.toml file.


