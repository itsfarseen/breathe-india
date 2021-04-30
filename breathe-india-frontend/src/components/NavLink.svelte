<script>
  import { Link } from "svelte-routing";
  export let to = "";

  export let klass = "";
  export let activeKlass = "";
  export let inactiveKlass = "";

  function getProps({ location, href, isPartiallyCurrent, isCurrent }) {
    const isActive = href === "/" ? isCurrent : isPartiallyCurrent || isCurrent;

    // The object returned here is spread on the anchor element's attributes
    if (isActive) {
      return { class: klass + " " + activeKlass };
    }
    return { class: klass + " " + inactiveKlass };
  }
</script>

<Link {to} {getProps}>
  <slot />
</Link>
