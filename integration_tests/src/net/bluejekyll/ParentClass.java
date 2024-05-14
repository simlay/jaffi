package net.bluejekyll;

public class ParentClass {
    public final int call_dad(int val) {
        return val;
    }

    // This is to test the `FromJavaToRust` and `IntoJavaValue` traits for Object.
    public final Object call_dad_object(Object val) {
        return val;
    }

    // In some cases, throwables are passed in and sometimes returned.
    // https://developer.android.com/reference/android/util/AndroidException#AndroidException(java.lang.String,%20java.lang.Throwable)
    public final Throwable call_dad_throwable(Throwable val) {
        return val;
    }

}
