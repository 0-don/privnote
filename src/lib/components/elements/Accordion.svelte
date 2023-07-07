<ul class="text-white shadow-2xl">
  <!-- single Accordion -->
  <li class="item bg-indigo-500">
    <!-- wrap is important here -->
    <input type="checkbox" id="faq1" class="peer appearance-none" />
    <i class="fa-solid fa-chevron-right inline peer-checked:hidden" />
    <i class="fa-solid fa-chevron-down hidden peer-checked:inline" />
    <label for="faq1" class="grow cursor-pointer py-4">
      <!-- note: grow is used to fill all remaining space of the article -->
      <p>What is your name?</p></label
    >
    <!-- note: checkbox will be triggered even if label is triggered. -->
    <div class="content">
      <!-- note: basis may not work without wrap -->
      <p>My name is Sohan Emon</p>
    </div>
  </li>
</ul>

  <!-- <div class="inline-flex cursor-pointer items-center rounded-md border border-zinc-500 bg-zinc-600 p-2">
    <i class="i-icomoon-free:question text-xl" />
  </div> -->
