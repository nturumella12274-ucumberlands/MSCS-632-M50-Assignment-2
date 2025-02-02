public class java_memory_management {
    public static void main(String[] args) {
        // Print initial memory usage
        printMemoryUsage("Before creating objects");

        // Create a large number of objects in a loop
        for(int i = 0; i < 1_000_000; i++) {
            Object objectCreated = new Object();
            // objectCreated is out of scope after each loop iteration,
            // so eventually these objects become eligible for GC.
        }

        // Print memory usage after object creation
        printMemoryUsage("After creating objects");

        // Request garbage collection (not guaranteed to run immediately)
        System.gc();

        // Optional: pause briefly to give the JVM time to perform GC
        try {
            Thread.sleep(2000);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        // Print memory usage after suggesting GC
        printMemoryUsage("After requesting GC");
    }

    /**
     * Utility method to print current heap usage.
     * totalMemory(): total memory in the Java Virtual Machine (JVM).
     * freeMemory(): free memory within the JVM.
     * usedMemory = totalMemory - freeMemory.
     */
    private static void printMemoryUsage(String phase) {
        long totalMemory = Runtime.getRuntime().totalMemory();
        long freeMemory = Runtime.getRuntime().freeMemory();
        long usedMemory = totalMemory - freeMemory;

        System.out.println(phase + " - Used memory: " + usedMemory + " bytes");
    }
}
