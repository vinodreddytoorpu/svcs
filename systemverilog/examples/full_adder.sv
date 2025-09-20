// Half Adder module
module half_adder(
    input a,
    input b,
    output sum,
    output carry
);
    assign sum = a ^ b;
    assign carry = a & b;
endmodule

// Full Adder using two Half Adders
module full_adder(
    input a,
    input b,
    input cin,
    output sum,
    output cout
);
    wire sum1, carry1, carry2;

    half_adder ha1(
        .a(a),
        .b(b),
        .sum(sum1),
        .carry(carry1)
    );

    half_adder ha2(
        .a(sum1),
        .b(cin),
        .sum(sum),
        .carry(carry2)
    );

    assign cout = carry1 | carry2;
endmodule
