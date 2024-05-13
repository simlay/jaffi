package net.bluejekyll;

public class ParentClass {
    public final int call_dad(int val) {
        return val;
    }

    // This is to test the `FromJavaToRust` and `IntoJavaValue` traits for Object.
    public final Object call_dad_object(Object val) {
        return val;
    }
}
