import './styles.css';
const rust = import('../pkg')

rust
.then( m => m.start())
.catch(console.error)

console.log("Test from type script")